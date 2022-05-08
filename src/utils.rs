use tui::text::{Span, Spans};

fn get_total_half_count(origin: String) -> usize {
    let mut count = 0;

    origin.chars().for_each(|c| {
        if c.is_ascii() {
            count += 1;
        } else {
            count += 2;
        }
    });

    count
}

// Splits a string into a vector of strings to appeal to a width (used for word wrap)
pub fn wrap_lines<'a>(origin: String, line_limit: usize) -> Vec<Spans<'a>> {
    // Case if "" is passed
    if origin.is_empty() {
        return vec![Spans::from(Span::raw(""))];
    }

    let total_chars = get_total_half_count(origin.clone());

    if total_chars <= line_limit {
        return vec![Spans::from(Span::raw(origin))];
    }

    // let mut subs: Vec<Spans> = Vec::with_capacity(origin.len() / line_limit);
    let mut subs: Vec<Spans> = Vec::new();

    let mut iter = origin.chars();

    loop {
        let mut line = String::with_capacity(line_limit);
        let mut pos = 0;
        while pos < line_limit {
            // new line
            if let Some(c) = iter.next() {
                if c.is_ascii() {
                    pos += 1;
                } else {
                    pos += 2;
                }
                line.push(c);
            } else {
                subs.push(Spans::from(Span::raw(line)));
                return subs;
            }
        }
        subs.push(Spans::from(Span::raw(line)));
    }
}

pub fn parse_unchecked(content: &str, index: usize) -> usize {
    let split_result = content.split(' ').into_iter().collect::<Vec<&str>>();

    split_result[index].parse::<usize>().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_wrap_lines() {
        let test_desc = "图解 TLS 连接。用在线交互的方式讲解 TLS 的全过程，从建立 TLS 1.2 客户端发送 ping 再到接收 pong，详细到每一个字节。";

        let lines = wrap_lines(test_desc.to_string(), 30);
        println!("{:?}", lines);
    }

    #[test]
    fn test_get_total_half_count() {
        let test_desc = "图解 TLS 连接。用在线交互的方式讲解 TLS 的全过程，从建立 TLS 1.2 客户端发送 ping 再到接收 pong，详细到每一个字节。";

        println!("{:?}", get_total_half_count(test_desc.to_string()));
    }
}
