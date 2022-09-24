use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootEmailRequest {
    pub personalizations: Vec<Personalization>,
    pub from: From,
    pub subject: String,
    pub content: Vec<Content>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Personalization {
    pub to: Vec<To>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct To {
    pub email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct From {
    pub email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: String,
}

/* pub async fn send_email(
    &self,
    recipient: SubscriberEmail,
    subject: &str,
    html_content: &str,
    text_content: &str,
) -> Result<(), reqwest::Error> {
    let url = format!("{}/email", self.base_url);
    /* let email_data = SendEmailRequest {
        from: self.sender.as_ref().to_owned(),
        to: recipient.as_ref().to_owned(),
        subject: subject.to_owned(),
        html_body: html_content.to_owned(),
    }; */

    let from = From {
        email: self.sender.as_ref().to_owned(),
    };

    //personalizations
    let to = To {
        email: recipient.as_ref().to_owned(),
    };
    let tos:Vec<To>/* Vec<To> */ = vec![to];
    let personalizations/* Vec<Personalization> */ = Personalization {to:tos};

    //content
    let value = html_content.to_owned();
    let type_field = String::from("text/html");
    let content = Content { type_field, value };

    let request_body = RootEmailRequest {
        personalizations: vec![personalizations],
        from,
        subject: subject.to_owned(),
        content: vec![content],
    };

    let builder = self
        .http_client
        .post(&url)
        .header(
            "Authorization: Bearer ",
            self.authorization_token.expose_secret(),
        )
        .json(&request_body)
        .send()
        .await?;
    Ok(())
} */
