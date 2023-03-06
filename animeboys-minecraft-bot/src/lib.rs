use anyhow::anyhow;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info};

const AUTHORIZED_USERS: [&str; 2] = ["Faults#1644", "ChrisW#6807"];

struct Bot {
    /// The instance id of the ec2 instance
    instance_id: String,
    /// The profile to use when connecting to aws
    aws_profile: String,
    ec2_client: aws_sdk_ec2::Client
}

impl Bot {
    pub async fn new(instance_id: String, aws_profile: String) -> Bot {
        // Create ec2 client
        let config = aws_config::load_from_env().await;
        let client = aws_sdk_ec2::Client::new(&config);
        Bot {
            instance_id,
            aws_profile,
            ec2_client: client
        }
    }
    pub async fn start_instance(&self) {
        todo!()
    }
    pub async fn stop_instance(&self) {
        todo!()
    }
    pub async fn get_instance_status(&self) {
        todo!()
    }
    pub async fn get_instance_ip(&self) {
        todo!()
    }
    pub async fn print_help(&self) -> String {
        "
    Welcome to the Minecraft Bot! Here are the commands you can use:
        $start - Starts the Minecraft server
        $stop - Stops the Minecraft server
        $status - Gets the status of the Minecraft server
        $help - Displays this message
        ".into()
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        // TODO: Add a way to authorize users
        // TODO: Add a way to only accept messages from a specific channel
        match msg.content.as_str() {
            "$start" => {
                if let Err(e) = msg.channel_id.say(&ctx.http, "Starting instance").await {
                    error!("Error sending message: {:?}", e);
                }
            }
            "$stop" => {
                if let Err(e) = msg.channel_id.say(&ctx.http, "Stopping instance").await {
                    error!("Error sending message: {:?}", e);
                }
            }
            "$status" => {
                if let Err(e) = msg.channel_id.say(&ctx.http, "Instance status").await {
                    error!("Error sending message: {:?}", e);
                }
            }
            "$getip" => {
                if let Err(e) = msg.channel_id.say(&ctx.http, "Getting instance ip").await {
                    error!("Error sending message: {:?}", e);
                }
            }
            "$help" => {
                if let Err(e) = msg.channel_id.say(&ctx.http, self.print_help().await).await {
                    error!("Error sending message: {:?}", e);
                }
            }
            _ => {}
        }
        if msg.content == "!hello" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
                error!("Error sending message: {:?}", e);
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
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };
    let instance_id = if let Some(instance_id) = secret_store.get("INSTANCE_ID") {
        instance_id
    } else {
        return Err(anyhow!("'INSTANCE_ID' was not found").into());
    };
    let aws_profile = if let Some(aws_profile) = secret_store.get("AWS_PROFILE") {
        aws_profile
    } else {
        return Err(anyhow!("'AWS_PROFILE' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot::new(instance_id, aws_profile).await)
        .await
        .expect("Err creating client");

    Ok(client)
}