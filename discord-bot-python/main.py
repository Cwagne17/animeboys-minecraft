import discord
import os
import sys
import dotenv
import logging
import boto3

dotenv.load_dotenv()

logger = logging.getLogger(__name__)

logging.basicConfig(   
    level=logging.INFO,
    format='[%(levelname)s] %(asctime)s - %(message)s',
    datefmt='%Y-%m-%d %H:%M:%S',
    handlers=[
        logging.FileHandler("output.log"),
        logging.StreamHandler()
    ]
)

AUTHORIZED_USERS = ['Faults#1644', 'ChrisW#6807']
PRIVATE_CHANNEL = "minecraft-bot"

KEY = os.environ.get("DISCORD_KEY")

if KEY is None:
    logger.info("Key not found")
    sys.exit(1)

INSTANCE_ID = os.environ.get("INSTANCE_ID")
if INSTANCE_ID is None:
    logger.error("Instance ID not found")
    sys.exit(1)

AWS_PROFILE = os.environ.get("AWS_PROFILE")
if AWS_PROFILE is None:
    logger.warning("AWS_PROFILE not found. Using default profile.")

session = boto3.Session(profile_name=AWS_PROFILE)
ec2 = session.client('ec2', region_name='us-east-1')

intents = discord.Intents.default()
intents.message_content = True

client = discord.Client(intents=intents)

@client.event
async def on_ready():
    logger.info('We have logged in as {0.user}'.format(client))

@client.event
async def on_message(message):
    ''' Event handler that executes when a message is sent in any channel of the discord server '''
    if message.author == client.user:
        return
    if str(message.channel) != PRIVATE_CHANNEL:
        logger.info("Unauthorized channel: " + str(message.channel))
        return
    if str(message.author) not in AUTHORIZED_USERS:
        logger.info("Unauthorized user: " + str(message.author))
        return

    command = message.content.split("$")
    if len(command) < 2:
        return

    command = command[1].lower()
    logger.info("Incoming command: " + command)
    if command == "start":
        await message.channel.send('Starting...')
        await message.channel.typing()
        await message.channel.send(start_minecraft_server())
    elif command == "stop":
        await message.channel.send('Stopping...')
        await message.channel.typing()
        await message.channel.send(stop_minecraft_server())
    elif command == "status":
        await message.channel.send('Getting status...')
        await message.channel.typing()
        await message.channel.send(get_minecraft_server_status())
    elif command == "help":
        await message.channel.send(print_help())
    else:
        await message.channel.send('Unknown command. Try $help for a list of commands.')
    

def print_help():
    return f"""
    Welcome to the Minecraft Bot! Here are the commands you can use:
    $start - Starts the Minecraft server
    $stop - Stops the Minecraft server
    $status - Gets the status of the Minecraft server
    $help - Displays this message
    """

def start_minecraft_server():
    try:
        res = ec2.start_instances(
            InstanceIds=[INSTANCE_ID],
            DryRun=False
        )
        curr_state = res['StartingInstances'][0]['CurrentState']['Name']
        return f"Server's state is now {curr_state}"
    except Exception as e:
        logger.error(e)
        return "An error occurred while starting the server. Please try again later."
    
def stop_minecraft_server():
    try:
        res = ec2.stop_instances(
            InstanceIds=[INSTANCE_ID],
            DryRun=False
        )
        curr_state = res['StoppingInstances'][0]['CurrentState']['Name']
        return f"Server's state is now {curr_state}"
    except Exception as e:
        logger.error(e)
        return "An error occurred while stopping the server. Please try again later."
    
def get_minecraft_server_status():
    try:
        res = ec2.describe_instances(
            DryRun=False
        )
        curr_state = res['Reservations'][0]['Instances'][0]['State']['Name']
        return f"Server's state is {curr_state}"
    except Exception as e:
        logger.error(e)
        return "An error occurred while getting the server's status. Please try again later."

client.run(KEY, log_handler=None)
