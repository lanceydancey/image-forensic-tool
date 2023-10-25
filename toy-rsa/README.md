###Lance Miller Exponential Toy-RSA

##The writeup

I tried to follow the logical order of the encryption algorithm when I was writing my functions. I started with genkey,
and verified that I got what looked like output matching two largge prime numbers. 
I moved onto encrypt and tested that.
Decrypt was a little more complicated. No matter what method I tried, I got a panic trying to convert the u64 message to u32. So,
I never really got to see if the logic within my function is correct. I had a hell of a week, so didnt get to this until after your last office hour,
otherwise I would have taken you up on some help here.

Had some small issues with clippy warnings vis-a-vis the library crate and a main function. 

Well, had a pretty silly error of thinking that I could multiply two u32's together and then convert to u64 afterwards. The overflow error i was getting could have been more specific i feel like, but maybe im just bitter. got it all working now. 

