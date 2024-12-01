/* 두 정수 A와 B를 입력받은 다음, A+B를 출력하는 프로그램을 작성하시오. */

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    let values: Vec<i32> = s
        .as_mut_str()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    println!("{}", values[0] + values[1]);
}

#[cfg(test)]
mod p1000 {
    use backjoon_rust::{get_bin_name, create_command, measure_execution_time};

    #[test]
    /* 첫째 줄에 A와 B가 주어진다. (0 < A, B < 10) */
    fn validate() -> anyhow::Result<()> {
        let bin_name = get_bin_name(file!());
        let mut cmd = create_command(&bin_name)?;
        let mut test_number: i32 = 1;
        
        for a in 1..10 {
            for b in 1..10 {
                let input = format!("{} {}", a, b);
                
                measure_execution_time(
                    || {
                        cmd.write_stdin(input)
                            .assert()
                            .stdout(format!("{}\n", a + b));
                        Ok(())
                    },
                    Some(test_number),
                )?;
                
                test_number += 1;
            }
        }
        Ok(())
    }
}