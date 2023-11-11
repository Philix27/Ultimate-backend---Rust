pub mod consts;
pub mod msg;


struct SmsParams {
     to: String,
     from: String,
     sms:String,
     type: String,
     channel: String,
     api_key:String,
}

trait SendSmsNotification {
    // fn sendSms(params: Params) -> String {
    //     "Send sms"
    // }
}


struct EmailParams {
     to: String,
     from: String,
     sms: String,
     template_id: String,
     channel: String,
     api_key:String,
}

trait SendEmailNotification {
    // fn sendSms(params: EmailParams) -> String {}
}