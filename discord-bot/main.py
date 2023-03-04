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

    if message.author == client.user:
        return
    if str(message.channel) != PRIVATE_CHANNEL:
        logger.info("Unauthorized channel: " + str(message.channel))
        return
    if str(message.author) not in AUTHORIZED_USERS:
        logger.info("Unauthorized user: " + str(message.author))
        return

    command = message.content.split("$")[1]
    if command == "start":
        await message.channel.send('Starting...')
        await message.channel.typing()
    if command == "hello":
        await message.channel.send('Hello!')

def start_minecraft_server():
    pass
    
KEY = os.environ.get("DISCORD_KEY")

if KEY is None:
    logger.info("Key not found")
    sys.exit(1)

client.run(KEY, log_handler=None)