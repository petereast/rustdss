// This will parse the incoming commands. I imagine this file is going to get pretty big, so
// eventually we're going to have to refactor it into something smaller.

// Commands come in the form of RESP arrays of bulk strings and look something like this:
/*

C: *2\r\n
C: $4\r\n
C: LLEN\r\n
C: $6\r\n
C: mylist\r\n

S: :48293\r\n

*/
