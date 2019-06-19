import 0x0.LibraAccount;
import 0x0.LibraCoin;


main(payee: address , amount: u64) {

// all the local variable declarations should in the starting of the procedure. 
let coin: R#LibraCoin.T;
// ( here R# represnets the Resource value and V# for unrestricted value).


// access the object Libracount.T resource and then sender balance is less than amount 


coin = LibraAccount.withdraw_from_sender(move(amount));
// will use the acquired LibraAccount.T resource and then getting the payee address and the money to be transferred

LibraAccount.deposit(move(payee) , move(coin) );



// must return ( hardcoded ) TODO :-  how you can return the obejct based storage and also how you can make complex recursive transactions.

return;
}


// now working on the interact with these modules and resources in a transaction , we need to avoid the 
// fatal errors ( like invalid address etc).



main(payee: address , amount: u64 )
{

let coin: R#LibraCoin.T;

let account_exists: bool;

// now doing the transaction and after putting the  safety check for 
coin = LibraAccount.withdraw_from_sender(move(amount));

account_exists = LibraAccount.exists(copy(payee));

//chekcing  implementation of the account_exists

if(!move(account_exists))
{   create_account(copy(payee));                          

}

LibraAccount.deposit(move(payee) , move(coin)); 

return;
}


// similarly for the multiple implementation of the transactions.

main (payee1: address , ....  payeeN: address (thats too clumsy , work on making an dynamic array passing in rust)   , amount: u64 .....)
{ 
// similarly define the multiple accounts for the  partner can be done by  getting the required implementation of dynamic arrays.


total = // summation of the amounts(amount1).

// now perform the payments ( get it with the array based implementation).
coin2 = LibraCoin.deposit()

}




// Writing the given custom  module (aka library for normal OOps programmers) for a  deferred  and custom payments 

module FutureLibraCoin {
import 0x0.LibraCoin;


//   now we will be describing the actual components of the resource.



resource T {

coin: R#LibraCoin.T,
recipient: address
// and so on :- add credit score , or 


}
// thus now here we will be creating a new  transaction token for the given investor that  we have just described as a wrapper


public create(coin: R#LibraCoin.T, recipient: address)
{ let t = R#Self.T;
// this is the creation of the transaction as a construct . only the procedures described in the given module can utilise the given construct.
t = T { coin : move(coin);
recipient: move(recipient);
};



// here we will be pulishing the given token under the given address . each account can be earmarked under the given address
// thus general function for transaction move_to_(name)(move(address_of_recipient)).

move_to_sender<T>(move(t));
return;

}
// here in order to utilise the given construct for a given function which is constituent of the given module , we need to inherit the function.
public  claim_for_recipient(earmarked_coin_address: address) : R#Self.T 
{
// also see how to do the reference of the given object to another DATATYPE object.
let t: R#Self.T;
let t_ref: &R#Self.T;
let sender: address;
//  now moving the designated coins to the given earmarked given coin address.
}


t = move_from<T>(move(earmarked_coin_address));

t_ref = &t;

}

// getitng the address of transaction sender and then verifying the integrity of the transmission process 
// TODO :- bring up the ZKSNARK magic.
sender = get_txn_sender();

// also we will be passing the code error (i.e 99 ) for comparison
// TO-DO in case of other error codes to be checked.
assert(*(&move(t_ref).recipient) == move(sender), 99);

return move(t);


}




// now at the receieving side , get the constituents and then return the  corresponding txn a sucees / fail

public unwrap( t: R#Self.T ): R#LibraCoin.T {

let coin: R#LibraCoin.T;
let recipient: address;



// it removes the outer resource  and returning the contents.

T {coin , recipient} = move(t);

return move(coin);



}


// voila , now to work on the future dev Exp :- we need to be getting the direct  changes on the consensus behaviour and make it more dynamic like solidity.






}









