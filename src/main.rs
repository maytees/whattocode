use serde::Deserialize;
extern crate question;
use question::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let client = reqwest::Client::builder().build()?;

        let res = client
            .get("https://what-to-code.com/api/ideas/random")
            .send()
            .await?;

        let jres = res.json::<Idea>().await?;
        println!("-------------------------------------------------------------");

        println!(
            "\n\nHere is an idea: \n{} \n{}\nThis idea has {} likes\n\n",
            jres.title, jres.description, jres.likes
        );

        println!("-------------------------------------------------------------");
        let resume = Question::new("Would you like a new idea?")
            .default(Answer::YES)
            .show_defaults()
            .confirm();

        if resume == Answer::YES {
            continue;
        } else {
            break;
        }
    }

    Ok(())
}

#[derive(Deserialize, Debug)]
pub struct Idea {
    title: String,
    description: String,
    likes: usize,
}
