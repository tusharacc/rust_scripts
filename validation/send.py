import sys,mail
import create_html as html
from datetime import datetime
import logging

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)
formatter = logging.Formatter('%(asctime)s - %(name)s - %(levelname)s - %(message)s')
handler = logging.FileHandler('Log/SCI_JIRA_VALIDATION_'+datetime.now().strftime('%Y%m%d')+'.log')
handler.setFormatter(formatter)
logger.addHandler(handler)

logger.info(f"Processing Started")
records = []
for line in sys.stdin:
    arr = line.split("$$$") 
    records.append(arr)
logger.info(f"The records is {records}")
markup = html.create_table(records,["Key","Summary","Owner"])
mail.send_mail (body=f"<html><style>table{{border:1px thin black;}}th{{background-color:black;color:white}}td{{border:1px solid silver;background-color:LIGHTSLATEGRAY;color:GAINSBORO}}tr{{border:1px solid dimgray;}}th{{border:1px solid lightgray;}}</style><body>{markup}</body></html>",recipient='test',subject=f"Small Commercial - Please Update RCA - {datetime.now().strftime('%Y-%m-%d')}")
logger.info(f"Ended")