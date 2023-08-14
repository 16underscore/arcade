#[cfg(feature = "client")]
mod client;
mod entity;
#[cfg(feature = "server")]
mod server;

fn main() {
	#[cfg(feature = "client")]
	client::main();
	#[cfg(feature = "server")]
	server::main();
}
