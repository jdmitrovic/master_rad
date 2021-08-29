fn povrsina_objekta<T>(objekat: &T)
    where T: Objekat2D + Display
{
    println!("Povrsina objekta {} je: {}", objekat, objekat.povrsina());
}
