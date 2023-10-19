fn add_space(s: &mut String,n: i32){
    let mut i=0;
    while i<n{
        s.push(' ');
        i+=1;
    }
}

fn add_str(s: &mut String,a: &str){
    s.push_str(a);
}

fn add_integer(s: &mut String,mut nr: i32){
    let mut m=0;
    let mut k=0;
    let mut zero=0;
    let mut ch;
    while nr!=0{
        m=m*10+(nr%10);
        if m==0{
            zero+=1;
        }
        nr=nr/10;
    }
    while m!=0{
        if k==3{
            s.push('_');
            k=0;
        }
        ch=((m%10)as u8 + b'0') as char;
        s.push(ch);
        m=m/10;
        k+=1;
    }
    while zero!=0{
        if k==3{
            s.push('_');
            k=0;
        }
        s.push('0');
        k+=1;
        zero-=1;
    }
}

fn add_float(s: &mut String,nr: f32){
    let mut n=nr as i32;
    add_integer(s, n);
    let mut m=nr - n as f32;
    m=m*1000.0;
    let mut m=m as i32;
    n=0;
    let mut k=0;
    while m!=0{
        n=n*10+(m%10);
        m=m/10;
        k+=1;
    }
    let mut ch;
    s.push('.');
    while k!=3{
        s.push('0');
        k+=1;
    }
    while n!=0{
        ch=((n%10)as u8 + b'0') as char;
        s.push(ch);
        n=n/10;
    }
}

fn main() {
    let mut s=String::from("");
    add_space(&mut s,40);
    add_str(&mut s, "I");
    add_space(&mut s,1);
    add_str(&mut s, "💚");
    add_str(&mut s, "\n");
    add_space(&mut s,40);
    add_str(&mut s, "RUST.");
    add_str(&mut s, "\n\n");
    add_space(&mut s,4);
    add_str(&mut s, "Must");
    add_space(&mut s,12);
    add_str(&mut s, "crate");
    add_space(&mut s,6);
    add_integer(&mut s, 306437968);
    add_space(&mut s,11);
    add_str(&mut s, "and");
    add_space(&mut s,5);
    add_str(&mut s, "lastest");
    add_space(&mut s,9);
    add_str(&mut s, "is");
    add_str(&mut s, "\n");
    add_space(&mut s,9);
    add_str(&mut s, "downloaded");
    add_space(&mut s,8);
    add_str(&mut s, "has");
    add_space(&mut s,13);
    add_str(&mut s, "downloads");
    add_space(&mut s,5);
    add_str(&mut s, "the");
    add_space(&mut s,9);
    add_str(&mut s, "version");
    add_space(&mut s,4);
    add_float(&mut s, 2.038);
    add_str(&mut s, ".\n");
    add_space(&mut s,20);
    print!("{s}");
    
}

