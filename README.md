# Rust Firewall

This is a basic example of a firewall in Rust. The firewall is capable of blocking or allowing traffic based on a user-defined list of rules.

## Project Structure

The project consists of a single file `main.rs`, which contains the firewall code.

## Main Components

### `FirewallRule`

The `FirewallRule` struct represents a firewall rule, with the following fields:
- `source_ip`: source IP address of the rule
- `destination_ip`: destination IP address of the rule
- `allow`: indicates whether the rule allows (`true`) or blocks (`false`) the traffic

### `check_packet`

The `check_packet` function checks whether a packet should be allowed or blocked based on the firewall rules. It iterates through the list of rules and returns `true` if it finds a matching rule that permits the packet.

### `simulate_incoming_traffic`

The `simulate_incoming_traffic` function simulates incoming traffic and prints whether a packet is allowed or blocked based on the firewall rules.

## Example Usage

The usage example in `main()` demonstrates how to create some firewall rules and simulate incoming traffic.

## How to Run

To run the code, simply clone the repository and compile the `main.rs` file using the Rust compiler.

