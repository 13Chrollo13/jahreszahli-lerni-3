use std::io;
use rand::{Rng, RngExt};

fn main() {
    let mut streak = 0;
    println!( "{}", '\u{1F988}' );
    loop {
        let mut zeit = "UwU";
        //random bereich
        let min = 1;
        let max = 7;
        let answer = rand::rng().random_range(min..=max);


        //fragen ausgabe
        match answer {
            1 => println!("wann fing die vorindustrlle zeit an"),
            2 => println!("wann wurde der deutsche zollverein gegründet"),
            3 => println!("wann startete die frühnindustrialiesierung"),
            4 => println!("wann waren die weber aufstände"),
            5 => println!("wann fing der gründerboom an"),
            6 => println!("wann fing die gründer kriese an"),
            7 => println!("wann fing der 1 weltkrieg an"),

            i32::MIN..=0_i32 | 2_i32..=i32::MAX => todo!()
        }


        //userinput

        let mut a = String::new();
        io::stdin().read_line(&mut a).expect("failed to readline");
        let a = a.trim();
        let userinput: i32 = a.parse().unwrap();
        let mut x = 1;

        // matchd jahreszahlen
        match answer {
            1 => x = 1815,
            2 => x = 1834,
            3 => x = 1835,
            4 => x = 1844,
            5 => x = 1871,
            6 => x = 1873,
            7 => x = 1914,
            _ => panic!(),
        };
        if x == userinput {
            streak += 1;
            if streak <= 3 {
                println!("supi ^w^")
            }
           if streak <= 6 && streak > 3 {
                println!("supi dupi OwO")
           }
           if streak > 6 {
               println!("UwU supi dui bissi echt guti {}", '\u{1F988}' )
           }


        }
        else {
            println!("schadi OwO");
            println!("die richtige antwort ist {}", x);
            streak = 0;

        }
    }
}



/*
1815 vorindustrielle zeit
1834 gründung des deutschen zollvereins
1835 frühindustrialiesirung
1844 weberaufstände
1871 gründerboom
1873 gründer krieses
1914 1 weltkrieg
 */
