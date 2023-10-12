###Lance Miller Exponential Modulo in Rust

##The writeup

The first thing that I did was convert the algorithm given to us in the hw assignment 
and tranlsate that into a rust function. Couple of pieces of syntax that were a little different from what I was used to, but no real surprises. I left this with a test case hard coded because I didnt have the comman line argument input written yet.

The next thing I did was write the code in main() to take in the user input from the command line. I used the template from the book, and it went smoothly. I swapped out hard coded for values for the values supplied by the user. Everything was working. 

The next thing I did was copy the test the provided test over from the hw assignment. My program did not pass the test. And this was the first time that Rust didn't give very explicit feedback about what went wrong. The compiler did however let me know that I had an overflow, and I had a suspicion that when we multipled the 2 u64's together, that was the problem. Also the asserts provided were using the max value of u64. Upon close examination of the hw assignment, it was mentioned that during the math phase, variables need to be converted from 64 to 128, so I implemented this and the tests passed. 

I did try to use the parsenum() method provided by the hw assignment, but I couldnt get it to work properly, so I stuck with the FromStr library that was. 

