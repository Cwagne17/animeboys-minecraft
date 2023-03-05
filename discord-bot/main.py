import discord
import os
import sys
import dotenv
import logging
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
    if command == "start":
        await message.channel.send('Starting...')
    elif command == "stop":
        await message.channel.send('Stopping...')
    elif command == "status":
        await message.channel.send('Server is running')
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
    pass
    
KEY = os.environ.get("DISCORD_KEY")

if KEY is None:
    logger.error("Key not found")
    sys.exit(1)

client.run(KEY, log_handler=None)