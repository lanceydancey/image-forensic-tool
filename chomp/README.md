Lance Miller

Intentions: resubmit this with Rustdoc, tests etc. Fix some of the stupid errors. delete this comment. 

To run the game, simply do a cargo run. You will be prompted to enter a board size. This should be two ints in this format 'r c'. Row then column with a space in between. The board will generate and display. Turns are taken using the same format. Leave the upper left square for you opponent to eat to win!

Ok. So at this point in time the game is not perfect. if you enter (1,1), (the poison block of the chocolate bar), the game breaks. so dont do that. Also, if the board size gets much above 6x6, it stalls out. I havent tested to see if this is just because of the recursive nature of the winning_move function if if something is off in my code. I think it's the former. I also dont have any formal tests written. It's only been hand tested. That being said, the game works, which is a huge win. 

What I struggled with: Writing the AI algorithm was by far the hardest part. I think that after reading it a couple times I thought that I understood it, but I didn't really until I started to dig into it. There was just enough going on in it that I couldnt hold it all in my head at one time, so I had to start tinkering, and it took a good bit of tinkering. 

Struggles cont'd: I think the funniest thing that happened was that I was having an off by one error when I took in a use move. I was convinced that only the column was off by one, and I couldnt figure it out, it didnt make any sense. But then I got some extra eyes on it, and both row and column were off by one and it was a super easy fix. go figure. 

More struggles: I think I struggled over all syntactly with the language. Heretofore I don't think that we had been asked to do too much, and I had kind of skated by a little bit. But I definitely got some practice in on this one. Taking stuff in from the user is always a pain. I glossed over the hint about the prompted crate before it was too late and I didn't feel like refactoring soemthing that was working. 
