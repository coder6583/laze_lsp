use serde_json::{json, Value};

use super::tokens::TokenList;

pub fn pos_to_lc(mut pos: usize, lines_size: &Vec<usize>) -> (usize, usize) {
    println!("{}, {:?}", pos, lines_size);
    let mut line = 0;
    for line_size in lines_size {
        if pos > *line_size {
            pos -= line_size + 1;
            line += 1;
        } else {
            println!("{:?}", (line, pos));
            return (line, pos);
        }
    }
    (line, pos)
}

pub fn to_semantic_tokens(tokenlist: TokenList, lines_size: &Vec<usize>) -> Value {
    let mut last_pos = (0, 0);
    let mut semantic_tokens = vec![];

    println!("{:?}", tokenlist);

    for token in tokenlist {
        let now_token_pos = pos_to_lc(token.pos.0, lines_size);
        if now_token_pos.0 >= last_pos.0 {
            semantic_tokens.push(now_token_pos.0 - last_pos.0);
            if now_token_pos.0 == last_pos.0 {
                if now_token_pos.1 >= last_pos.1 {
                    semantic_tokens.push(now_token_pos.1 - last_pos.1);
                } else {
                    println!("Tokens are overlapping.");
                }
            } else {
                semantic_tokens.push(now_token_pos.1);
            }
        } else {
            println!("Tokens are overlapping.");
        }
        semantic_tokens.push(token.pos.1 - token.pos.0);
        semantic_tokens.push(token.token_type.into_usize());
        semantic_tokens.push(1usize << token.modifier.into_usize() as u32);
        last_pos = now_token_pos;
    }
    json!(semantic_tokens)
}
