use std::io;

fn print_map(map: &mut [[u8; 3]]) {
    for n in 0..=2{
	for o in 0..=2{
		if o == 2{
			if map[o][n] == 0{	
				println!(" .");
			}
			else if map[o][n] == 1{
				println!(" {}","X");		
			}
			else if map[o][n] == 2 {
				println!(" {}","O");
			}
		}
		else {
			if map[o][n] == 0{	
				print!(" . | ");
			}
			else if map[o][n] == 1{
				print!(" {} | ","X");		
			}
			else if map[o][n] == 2 {
				print!(" {} | ","O");
			}
		}
	}
	if n!=2{
		println!("--------------");
	}
    }
}

fn ai_play(map: &mut [[u8; 3]], deep: i32){
	let mut max: i32 = -10000;
	let mut tmp:i32;
	let mut maxo:usize = 0;
	let mut maxn:usize=0;
	for n in 0..=2{
		for o in 0..=2{
			if map[n][o]==0 {
				map[n][o]=1;
				tmp=Min(map, deep-1);
				//println!("DEBUG -> {}",tmp);
				//print_map(map);
				if tmp>max{
					maxo=o;
					maxn=n;
					max=tmp;
				}
				//println!("DEBUG");
				//print_map(map);
				map[n][o]=0;
			}
		}
	}
	map[maxn][maxo]=2;
}

fn Min(map: &mut [[u8;3]], deep: i32)-> i32{
	let  check:i32 = win(map);
	if deep==0 ||check != 0 {
		return eval(map);
	}
	let mut min:i32 = 10000;
	let mut tmp:i32;
	for n in 0..=2{
		for o in 0..=2{
			if map[n][o]==0{
				map[n][o]=1;

				tmp=Max(map, deep -1);
				if tmp <min{
					min=tmp;
				}

				map[n][o]=0;

			}
		}
	}
	return min;
}

fn Max(map: &mut [[u8;3]], deep: i32)-> i32{
	let check:i32 = win(map);
	if check != 0 || deep == 0 {
		return eval(map);
	}

	let mut max:i32 = -10000;
	let mut tmp:i32;
	for n in 0..=2{
		for o in 0..=2{
			if map[n][o]==0{
				map[n][o]=2;
				tmp=Min(map, deep -1);
				if tmp > max{
					max=tmp;
				}
				map[n][o]=0;
			}
		}
	}
	return max;
}

fn win(map: &mut [[u8;3]])->i32{
	let mut check : i32=0;
	for n in 0..=2{
		for o in 0..=2{
			if map[o][n]==0{
				check=check+1;
			}
		}
	} // Test all possibility to win
	if(map[0][0]==1 && map[0][1]==1 && map[0][2]==1) || (map[1][0]==1 && map[1][1]==1 && map[1][2]==1)||(map[2][0]==1 && map[2][1]==1 && map[2][2]==1)||(map[0][0]==1 && map[1][0]==1 && map[2][0]==1)||(map[0][1]==1 && map[1][1]==1 && map[2][1]==1)||(map[0][2]==1 && map[1][2]==1 && map[2][2]==1)||(map[0][0]==1 && map[1][1]==1 && map[2][2]==1)||(map[0][2]==1 && map[1][1]==1 && map[2][0]==1){
		return 1;
	}
	else if(map[0][0]==2 && map[0][1]==2 && map[0][2]==2) || (map[1][0]==2 && map[1][1]==2 && map[1][2]==2)||(map[2][0]==2 && map[2][1]==2 && map[2][2]==2)||(map[0][0]==2 && map[1][0]==2 && map[2][0]==2)||(map[0][1]==2 && map[1][1]==2 && map[2][1]==2)||(map[0][2]==2 && map[1][2]==2 && map[2][2]==2)||(map[0][0]==2 && map[1][1]==2 && map[2][2]==2)||(map[0][2]==2 && map[1][1]==2 && map[2][0]==2) {
		return 2;
	}
	if check == 0 {
		return 3;
	}
	else{
		return 0;
	}
}

fn eval(map: &mut [[u8;3]])->i32{
	let check : i32= win(map); //check if the game is over
	let mut cmpt:i32=0;
	let mut cmp1:i32 = 0;
	let mut cmp2:i32 =0;
	let mut cmpgen1:i32 = 0;
	let mut cmpgen2:i32 = 0;

	if check != 0 {
		for n in 0..=2{
			for o in 0..=2{
				if map[o][n] != 0 {
					cmpt=cmpt+1; //count number of turn played
				}
			}
		}
		if check == 1 {
			return 1000-cmpt;
		}
		else if check == 2{
			return -1000+cmpt;
		}
	}
	else{

		for i in 0..=2{ //down diag
			if map[i][i] == 1{
				cmp1=cmp1+1;
				cmp2=0;
				if cmp1==2{
					cmpgen1=cmpgen1+1;
				}
			}
			else if map[i][i]==2{
				cmp2=cmp2+1;
				cmp1=0;
				if cmp2==2{
					cmpgen2=cmpgen2+1;
				}
			}
		}
		for i in 0..=2{ //up diag
			if map[i][2-i] == 1{
				cmp1=cmp1+1;
				cmp2=0;
				if cmp1==2{
					cmpgen1=cmpgen1+1;
				}
			}
				else if map[i][2-i]==2{
					cmp2=cmp2+1;
					cmp1=0;
					if cmp2==2{
						cmpgen2=cmpgen2+1;
					}
				}
		}
		for o in 0..=2{
			cmp1=0;
			cmp2=0;
			for n in 0..=2{
				if map[o][n] == 1{
					cmp1=cmp1+1;
					cmp2=0;
					if cmp1==2{
						cmpgen1=cmpgen1+1;
					}
				}
					else if map[o][n]==2{
						cmp2=cmp2+1;
						cmp1=0;
						if cmp2==2{
							cmpgen2=cmpgen2+1;
						}
					}
			}
			cmp1=0;
			cmp2=0;
			for n in 0..=2{
				if map[n][o] == 1{
					cmp1=cmp1+1;
					cmp2=0;
					if cmp1==2{
						cmpgen1=cmpgen1+1;
					}
				}
					else if map[n][o]==2{
						cmp2=cmp2+1;
						cmp1=0;
						if cmp2==2{
							cmpgen2=cmpgen2+1;
						}
					}
			}
		}
	}
	//println!("DEBUG eval -> {}",cmpgen2-cmpgen1);

	return cmpgen2-cmpgen1;

}

//----------------
//MAIN
//----------------
fn main() {
    let mut x: [[u8; 3]; 3] = [[0; 3]; 3];
	let mut end:i32=0;
	println!("MORPION");
	println!("Wich difficulty ? (3,5 or 8)");
	let mut diff = String::new();
	io::stdin().read_line(&mut diff).unwrap();
	let mut diff_int: i32 = diff.trim().parse().unwrap();


	while end == 0 {
		print!("{}[2J", 27 as char); //Clean the terminal


		println!("Here is the map");
		print_map(&mut x);
		println!("Where did you want to play ?");

		println!("Column ->");
		let mut col = String::new();
		io::stdin().read_line(&mut col).unwrap();
		let col_int: usize = col.trim().parse().unwrap();

		println!("Line ->");
		let mut lin = String::new();
		io::stdin().read_line(&mut lin).unwrap();
		let lin_int: usize = lin.trim().parse().unwrap();

		x[col_int][lin_int] = 1;
		end=win(&mut x);
		if end==0 {
			ai_play(&mut x, diff_int);
			end=win(&mut x);
		}
		print_map(&mut x);
	}
	if end == 1 {
		print!("{}[2J", 27 as char); //Clean the terminal
		println!("YOU WIN !");
		print_map(&mut x);
	}
	else if end == 2{
		print!("{}[2J", 27 as char); //Clean the terminal
		println!("You loose");
		print_map(&mut x);
	}
	else{
		print!("{}[2J", 27 as char); //Clean the terminal
		println!("A draw");
		print_map(&mut x);
	}
}
