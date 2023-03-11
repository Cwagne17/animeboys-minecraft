use std::time::Duration;
use std::{fmt::Display};
use std::env;
use anyhow::anyhow;
use aws_sdk_ec2::Region;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info, debug};

#[derive(Debug)]
struct Ec2Error {
    message: String
}
impl Display for Ec2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl Ec2Error {
    pub fn new(message: String) -> Ec2Error {
        Ec2Error {
            message
        }
    }
}
impl std::error::Error for Ec2Error {}

const AUTHORIZED_USERS: [&str; 2] = ["Faults#1644", "ChrisW#6807"];
const CHANNEL_ID: u64 = 1081598245358276721;
struct Bot {
    /// The instance id of the ec2 instance
    instance_id: String,
    /// The profile to use when connecting to aws
    ec2_client: aws_sdk_ec2::Client
}

impl Bot {
    pub async fn new(instance_id: String) -> Bot {
        // Create ec2 client
        let config = aws_config::from_env().region(Region::new("us-east-1")).load().await;
        let env_config = aws_config::environment::credentials::EnvironmentVariableCredentialsProvider::new();
        let ec2_config_builder = aws_sdk_ec2::config::Builder::from(&config)
        .credentials_provider(env_config)
        .build();
        let client = aws_sdk_ec2::Client::from_conf(ec2_config_builder);
        Bot {
            instance_id,
            ec2_client: client
        }
    }
    pub async fn start_instance(&self) -> Result<String, Ec2Error> {
        let res = self.ec2_client
                .start_instances()
                .instance_ids(self.instance_id.clone())
                .send()
                .await
                .map_err(|e| Ec2Error::new(e.to_string()))?;
        Ok(res.starting_instances()
            .unwrap()[0]
            .current_state()
            .unwrap()
            .name()
            .unwrap()
            .as_str()
            .to_string())
    }
    pub async fn stop_instance(&self) -> Result<(), Ec2Error> {
        self.ec2_client
            .stop_instances()
            .instance_ids(self.instance_id.clone())
            .send()
            .await
            .map_err(|e| Ec2Error::new(e.to_string()))?;
        Ok(())
    }
    pub async fn get_instance_status(&self) -> Result<String, Ec2Error> {
        let res = self
            .ec2_client
            .describe_instances()
            .instance_ids(self.instance_id.clone())
            .send()
            .await
            .map_err(|e| Ec2Error::new(e.to_string()))?;
        let status = res
            .reservations()
            .ok_or(Ec2Error::new("No instances found".into()))?[0]
            .instances()
            .ok_or(Ec2Error::new("No instances found".into()))?[0]
            .state()
            .ok_or(Ec2Error::new("No state found".into()))?
            .name()
            .ok_or(Ec2Error::new("No name found".into()))?
            .as_str()
            .to_string();
        Ok(status)
    }
    pub async fn get_instance_ip(&self) -> Result<String, Ec2Error> {
        let status = self.get_instance_status().await?;
        if status != "running" {
            return Err(Ec2Error::new("Instance is not running".into()))
        }
        Ok(self.ec2_client
            .describe_instances()
            .instance_ids(self.instance_id.clone())
            .send()
            .await
            .map_err(|e| Ec2Error::new(e.to_string()))?
            .reservations()
            .ok_or(Ec2Error::new("No reservations found".into()))?[0]
            .instances()
            .ok_or(Ec2Error::new("No instances found".into()))?[0]
            .public_ip_address()
            .ok_or(Ec2Error::new("No public ip found".into()))?
            .to_string())
    }
    pub async fn print_help(&self) -> String {
        "
    Welcome to the Minecraft Bot! Here are the commands you can use:
        $start - Starts the Minecraft server (REQUIRES AUTHORIZATION)
        $stop - Stops the Minecraft server (REQUIRES AUTHORIZATION)
        $status - Gets the status of the Minecraft server
        $getip - Gets the public ip of the Minecraft server
        $help - Displays this message
        ".into()
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        // Ignore messages from self
        if msg.author.bot {
            return;
        }
        // TODO: Add a way to only accept messages from a specific channel
        if msg.channel_id.0 != CHANNEL_ID {
            info!("Message sent in wrong channel");
            return;
        }
        // TODO: Add a way to authorize users
        if !AUTHORIZED_USERS.contains(&msg.author.tag().as_str()) {
            if let Err(e) = msg.channel_id.say(&ctx.http, "You are not authorized to use this bot").await {
                error!("Error sending message: {:?}", e);
            }
            return;
        }
        debug!("Message received from {}", msg.author.tag());
        
        debug!("Message received: {}", msg.content);
        match (msg.content.as_str(), AUTHORIZED_USERS.contains(&msg.author.tag().as_str())) {
            ("$start", true) => {
                if let Err(e) = msg.channel_id.say(&ctx.http, "Starting instance...").await {
                    error!("Error sending message: {:?}", e);
                }
                let typing = msg.channel_id.start_typing(&ctx.http).unwrap();
                let status = self.start_instance().await;
                if status.is_err() {
                    msg.channel_id.say(&ctx.http, "Error starting instance").await.unwrap();
                    return;
                }
                let status = status.unwrap_or_else(|_| "undefined. Check Logs.".into());
                if let Err(e) = msg.channel_id.say(&ctx.http, format!("The status of the instance is now: {}", status)).await {
                    error!("Error sending message: {:?}", e);
                }
                // TODO: Get the public ip of the instance and send it to the user
                if let Err(e) = msg.channel_id.say(&ctx.http, "Getting public ip...").await {
                    error!("Error sending message: {:?}", e);
                }
                loop {
                    let status = self.get_instance_status().await;
                    if status.is_err() {
                        msg.channel_id.say(&ctx.http, "Error getting instance status").await.unwrap();
                        return;
                    }
                    let status = status.unwrap_or_else(|_| "undefined. Check Logs.".into());
                    if status == "running" {
                        let ip = self.get_instance_ip().await;
                        if ip.is_err() {
                            msg.channel_id.say(&ctx.http, "Error getting instance ip").await.unwrap();
                            return;
                        }
                        let ip = ip.unwrap_or_else(|_| "undefined. Check Logs.".into());
                        if let Err(e) = msg.channel_id.say(&ctx.http, format!("The public ip of the instance is: {}", ip)).await {
                            error!("Error sending message: {:?}", e);
                        }
                        break;
                    }
                    tokio::time::sleep(Duration::from_secs(3)).await;
                }
                typing.stop().unwrap();
                    
            }
            ("$stop", true) => {
                if let Err(e) = msg.channel_id.say(&ctx.http, "Stopping instance...").await {
                    error!("Error sending message: {:?}", e);
                }
                let typing = msg.channel_id.start_typing(&ctx.http).unwrap();
                if self.stop_instance().await.is_err() {
                    msg.channel_id.say(&ctx.http, "Error stopping instance").await.unwrap();
                    return;
                }
                if let Err(e) = msg.channel_id.say(&ctx.http, "The instance has been stopped").await {
                    error!("Error sending message: {:?}", e);
                }
                typing.stop().unwrap();
            }
            ("$status", _) => {
                let status = self.get_instance_status().await;
                if status.is_err() {
                    msg.channel_id.say(&ctx.http, "Error getting instance status").await.unwrap();
                    return;
                }
                let status = status.unwrap_or_else(|_| "undefined. Check Logs.".into());
                if let Err(e) = msg.channel_id.say(&ctx.http, format!("The status of the instance is: {}", status)).await {
                    error!("Error sending message: {:?}", e);
                }
            }
            ("$getip", _) => {
                msg.channel_id.say(&ctx.http, "Getting instance ip...").await.unwrap();
                let typing = msg.channel_id.start_typing(&ctx.http).unwrap();
                let ip = self.get_instance_ip().await;
                if ip.is_err() {
                    let ip = ip.unwrap_err();
                    msg.channel_id.say(&ctx.http, format!("Error getting instance ip: {}", ip.message)).await.unwrap();
                    return;
                }
                let ip = ip.unwrap_or_else(|_| "undefined. Check Logs.".into());
                if let Err(e) = msg.channel_id.say(&ctx.http, format!("The ip of the instance is: {}", ip)).await {
                    error!("Error sending message: {:?}", e);
                }
                if let Err(e) = typing.stop().ok_or("error stopping typing") {
                    error!("Error stopping typing: {:?}", e);
                }
            }
            ("$help", _) => {
                if let Err(e) = msg.channel_id.say(&ctx.http, self.print_help().await).await {
                    error!("Error sending message: {:?}", e);
                }
            }
            ("$hi", _) => {
                if let Err(e) = msg.channel_id.say(&ctx.http, "Hello!").await {
                    error!("Error sending message: {:?}", e);
                }
            }
            (message, is_auth) => {
                if message.strip_prefix("$").is_none() {
                    return;
                } else if (message == "$start" || message == "$stop") && !is_auth {
                    if let Err(e) = msg.channel_id.say(&ctx.http, "You are not authorized to use this command.").await {
                        error!("Error sending message: {:?}", e);
                    }
                    return;
                }
                if let Err(e) = msg.channel_id.say(&ctx.http, "Unknown command. Try $help for a list of commands.").await {
                    error!("Error sending message: {:?}", e);
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_service::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_service::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    env::set_var("AWS_ACCESS_KEY_ID", secret_store.get("AWS_ACCESS_KEY_ID").unwrap_or_default());
    env::set_var("AWS_SECRET_ACCESS_KEY", secret_store.get("AWS_SECRET_ACCESS_KEY").unwrap_or_default());
    env::set_var("AWS_DEFAULT_REGION", "us-east-1");
    let token = secret_store.get("DISCORD_TOKEN").ok_or_else(|| anyhow!("'DISCORD_TOKEN' was not found"))?;
    let instance_id = secret_store.get("INSTANCE_ID").ok_or_else(|| anyhow!("'INSTANCE_ID' was not found"))?;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot::new(instance_id).await)
        .await
        .expect("Err creating client");

    Ok(client)
}