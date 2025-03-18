fn main () {
    let x = 10;
    let y = x;
    println("Valor de x {} Valor de y {}", x, y);

    let mut z = 20;
    {

        let w = &mut z;
        *w +=  5;

        println("Novo valor de z: {}", z);
    }


}