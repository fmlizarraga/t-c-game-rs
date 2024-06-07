/*
Port of "Secret weapon" BASIC game
from "Computer Battlegames" book 1982
for ZX Spectrum and other microcomputers
original BASIC code

10 PRINT "TRAITOR'S CASTLE"
20 LET S=0

30 FOR G=1 TO 10

40 LET R$=""
50 LET T=INT(RND*9+1)

60 FOR L=1 TO 9
70 IF L=T THEN LET R$=R$+"O"
80 IF L<>T THEN LET R$=r$+"."
90 NEXT L

100 PRINT R$,

110 FOR I=1 TO 12
120 LET I$=INKEY$

130 IF VAL("O"+I$)=T THEN GOTO 170
140 NEXT I

150 PRINT "MISSED"
160 GOTO 190
170 PRINT "GOOD SHOT"
180 LET S=S+1

190 NEXT G

200 PRINT "YOU HIT ";S;"OUT OF 10"
210 STOP

*/

use std::time::Duration;
use crossterm::{event::{poll, read, Event, KeyCode}, terminal};
use rand::Rng;
use clearscreen;

fn main() -> std::io::Result<()> {
    clearscreen::clear().expect("Couldn't clear the screen");

    println!("TRAITOR'S CASTLE");

    let mut score = 0;
    
    terminal::enable_raw_mode()?;

    for _goes in 0..10 {
        let mut row = String::new();
        let target = rand::thread_rng().gen_range(1..=9);

        for i in 1..=9 {
            if i == target {
                row.push('O');
            }
            else {
                row.push('.');
            }
            row.push(' ');
        }

        println!("\n\r{}", row);


        if poll(Duration::from_secs(2))? {
            if let Event::Key(key_event) = read()? {
                match key_event.code {
                    KeyCode::Char(c) if c.is_ascii_digit() => {
                        let num: i32 = c.to_digit(10).unwrap() as i32;
                        if num == target {
                            println!("\n\rGood shot!");
                            score += 1;
                        }
                        else {
                            println!("\n\rMissed!");
                        }
                    }
                    _ => {
                        println!("\n\rMissed!");
                    }
                }
            }
        }
        else {
            println!("\n\rMissed!");
        }
    }

    terminal::disable_raw_mode()?;

    println!("\nYou hit {} out of 10",score);

    Ok(())
}
