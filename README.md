# rbot

Welcome to **rbot**, the official Rust library for programming robots in the exciting game **Bot Beats**. This crate provides all the necessary tools and functions to create and control your robots, allowing you to immerse yourself in the dynamic and competitive world of robotic programming.

## About Bot Beats

**Bot Beats** is an innovative programming game where players design and code robots to compete in various multiplayer battles. The game combines the thrill of coding with the excitement of robot battles, making it a perfect platform for both beginners and experienced programmers to test and evolve their skills.

## Features

- **Comprehensive API**: Access a wide range of functions to control your robot's behavior.
- **Competitive Challenges**: Compete against other players in robot battles and tournaments.
- **Extensive Documentation**: Detailed documentation and examples to help you get started quickly.

## Getting Started

To start your journey with **Bot Beats**, follow these steps:

1. **Download the Game**: Visit [https://botbeats.net](https://botbeats.net) to download the game. For the easiest installation, get it through Steam.

2. **Create an Account**: Once you've installed the game, launch it and create a user account.

3. **Create a Rust Robot**: After setting up your account, you can create a new Rust robot to begin programming and participating in challenges.

4. **Follow Our Tutorials**: Enhance your skills and learn the basics with our official tutorials (coming soon).

By following these steps, you'll be ready to dive into the world of **Bot Beats** and start programming your own robots.

### Example

Here’s a simple example to get you started. This program makes your robot move upward, uses the radar module to locate an enemy robot, and then fires a component towards it. The #![allow(unused_must_use)] attribute is used to suppress warnings about unused results from certain function calls.

```rust
#![allow(unused_must_use)]
use rbot;

pub fn main() {
    // Move the robot upward with the specified velocity
    rbot::velocity(0.0, 1.0, 1.0);

    // Wait for the Radar module to become available
    rbot::modules::await_module(rbot::modules::Module::Radar);

    // Retrieve the radar message, which contains the position of the enemy robot
    let radar_msg = rbot::modules::radar().expect("failed to get radar message");

    // Convert the enemy's position (x, y coordinates) to an angle
    let angle = rbot::conversions::xy_to_angle(radar_msg.x, radar_msg.y);

    // Aim at the enemy robot using the calculated angle
    rbot::await_aim(2, angle, 0.5);

    // Wait for the component to be ready
    rbot::await_component(2);

    // Fire the component towards the enemy robot
    rbot::use_component(2, false);
}
```

## Documentation

Comprehensive documentation for the **rbot** crate is available at [docs.rs/rbot](https://docs.rs/rbot). Here you can find detailed descriptions of all functions, modules, and examples to guide you through the development of your robot programs.

## Support

If you encounter any issues, have questions, or need further assistance, please reach out to us on our Discord channel. You can find the link to our Discord server on our [website](https://botbeats.net). We are here to help and look forward to seeing the amazing robots you create!

## Contributing

We welcome contributions from everyone! To ensure consistency, please adhere to the existing code style in the repository. Here are some guidelines to keep in mind:

- Code Style: Match the current code base style.
- Simplicity: We like simple functions with clear, informative docstrings.
- Avoid Overcomplication: Only introduce complexity when absolutely necessary [grub](https://grugbrain.dev/).
- Multi-Paradigm Approach: We believe in using the right tool for the job. Adapt your solution to the problem, rather than forcing the problem to fit a particular tool or paradigm.

The repository is hosted on GitHub: [https://github.com/Brotir/rbot](https://github.com/Brotir/rbot).

Thank you for your interest in contributing!

## License

**rbot** is licensed under the Apache 2.0 License.

---

Dive into the world of **Bot Beats** and start programming your robots today! Whether you're a seasoned coder or just getting started, **rbot** provides the tools and support you need to bring your robotic creations to life. Happy coding!
