pub struct MsgSender {
     to: String,
     from: String,
     sms:String,
     type: String,
     channel: String,
     api_key:String,
     media: Option<Media>
 }

 pub struct Media  {
      url: String,
      caption: String
  }    


const end_point = "https://api.ng.termii.com/api/sms/send";

