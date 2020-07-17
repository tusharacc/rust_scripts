from mailer import Mailer
from mailer import Message

recipients = {
    'pending_issance_wc':['tushar.saurabh2@chubb.com','Prashant.Thankachan@Chubb.com','aostroski@chubb.com','balaji.srinivasan@chubb.com','Saranya.Bangarusamy@Chubb.com','Hema.Dalvi@Chubb.com','tanand@chubb.com'],
    'icc_booking_wc':['tushar.saurabh2@chubb.com','Prashant.Thankachan@Chubb.com','Saranya.Bangarusamy@Chubb.com','tanand@chubb.com','SCIITWCProdSupport@Chubb.com'],
    'bcws_recon':['tushar.saurabh2@chubb.com','Prashant.Thankachan@Chubb.com','Saranya.Bangarusamy@Chubb.com','tanand@chubb.com','SCIITWCProdSupport@Chubb.com'],
    'unassigned':['SCIITProdSupport2@chubb.com','SciMarketL2Not@chubb.com'],
    'test':[ 'tushar.saurabh2@chubb.com'],
    'jira_validation':['SCIITProdSupport2@chubb.com']
}

def send_mail(body,recipient,subject):
    message = Message(From="SmallCommercialQRT@Chubb.com",To=recipients[recipient],charset="utf-8")                    
    message.Subject = subject
    message.Html = body

    try:
        sender = Mailer('mail.chubb.com')
        sender.send(message)
        return True
    except Exception as ex:
        return False