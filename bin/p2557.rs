/* Hello World!를 출력하시오. */

fn main() {
    println!("Hello World!");
}

#[cfg(test)]
mod p2557 {
    use backjoon_rust::{get_bin_name, create_command, measure_execution_time};

    #[test]
    fn validate() -> anyhow::Result<()> {
        let bin_name = get_bin_name(file!());
        let mut cmd = create_command(&bin_name)?;
        let result = measure_execution_time(|| {
            cmd.assert().stdout("Hello World!\n");
            Ok(())
        }, None);
        result
    }
}