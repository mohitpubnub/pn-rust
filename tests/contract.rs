use cucumber::{ given, when, then, World };
use std::{time::Duration, pin::Pin};

#[derive(Debug, Default)]
struct Keyset {
  pub subkey: String,
  pub pubkey: String
}

#[derive(Debug, Default, World)]
pub struct PubnubWorld {
    keyset: Keyset,
}

#[given("the demo keyset")]
fn set_keyset(world: &mut PubnubWorld){
  world.keyset.pubkey = "demo".to_string();
  world.keyset.subkey = "demo".to_string();
  println!("{:?}", world.keyset);
}

#[given("a message")]
fn message_defined(_world: &mut PubnubWorld) {
    // world.param.message = "hello pubnub";
}

#[when("I publish a message")]
fn pubnub_publish(world: &mut PubnubWorld) {
    // world.pubnub.publish(params);
    println!("Publish a message");
}

#[then("I get a timetoken")]
fn check_timetoken(world: &mut PubnubWorld) {
    // world.lastresponse = '1234'  // assert_eq
}

async fn fish(scenario: &cucumber::gherkin::Scenario) {
  let tagvalue = scenario.tags.first().unwrap();
  print!("Scenario TAG Value= {:?}", tagvalue);
}

#[tokio::main]
async fn main() {
PubnubWorld::cucumber()
    .before(|_feature, _rule, _scenario, _world| {
      // TODO: Get the value of tags from scenarios and make http call to tomato
    futures::FutureExt::boxed(async move { 
      fish(_scenario).await;})
    
    })
    .run_and_exit("tests/features/publish").await;

    // futures::executor::block_on(PubnubWorld::run("tests/features/publish"));
}
