#[cfg(test)]
mod tests {
    #[test]
    fn specification_example_test() {
        use assert_cmd::Command;

        let hex_encoded_input = "129df964b701d0b8e72fe7224cc71643cf8e000d122e72f742747708f5e3bb6294c619604e52dcd8f5446da7e9ff7459d1d3cefbcc231dd4c02730a22af9880c";

        let mut cmd =
            Command::cargo_bin("simple-pow").expect("Name of the executable should match spec");

        cmd.arg(hex_encoded_input)
            .assert()
            .success()
            .stdout("6681edd1d36af256c615bf6dcfcda03c282c3e0871bd75564458d77c529dcafe\n00003997\n");
    }
}
