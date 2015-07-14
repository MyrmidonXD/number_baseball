import random

def gen_rand_num():
    num = []
    i = 0
    while i < 4:
        x = random.randint(0,9)
        if x not in num:
            num.append(x)
            i += 1

    return num

def num_baseball(target):
    print "==============================================="
    print "Your input number should be a four-digit number"
    print "which has all different digits."
    print "==============================================="

    while True:
        guess = raw_input("Please enter your guess. : ")
        guess_int = []
        invalid_input = False

        if len(guess) != 4:
            invalid_input = True
        
        for c in guess:
            if (c.isdigit()) and (int(c) not in guess_int):
                guess_int.append(int(c))
            else:
                invalid_input = True
                break

        if invalid_input:
            print "Invalid input. Input must be a four-digit number and digits must be all different."
            continue

        strike = 0
        ball = 0

        for i in guess_int:
            if i in target:
                if target.index(i) == guess_int.index(i):
                    strike += 1
                else:
                    ball += 1

        if strike == 4:
            print "\nCongratulations! You guessed the right answer! \
The answer is {0}{1}{2}{3}.\n".format(target[0], target[1], target[2], target[3])

            restart = raw_input("Play again? (Y/N) : ")
            if restart == "Y" or restart == "y":
                return True
            else:
                return False
        else:
            print "{s} Strike(s), {b} Ball(s)".format(s=strike, b=ball)



playing = True

while playing:
    target_num = gen_rand_num()
    playing = num_baseball(target_num)

