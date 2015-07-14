#include <stdio.h>
#include <stdlib.h>
#include <time.h>

int* pickRandNum();
int numBaseball(int* targetNum);

int main(int argc, char** argv)
{
	int* targetNum = NULL;
	int gamePlaying = 1;

	srand(time(NULL));
	while(gamePlaying)
	{
		// Magic Happens!
		targetNum = pickRandNum();

		gamePlaying = numBaseball(targetNum);

		free(targetNum);
	}

	return 0;
}

int* pickRandNum()
{
	int* target = malloc(4 * sizeof(int));
	int repickFlag = 0;
	int i = 0, j = 0;

	for(i = 0; i < 4; i++)
	{
		target[i] = rand() % 10;

		for(j = 0; j < i; j++)
		{
			if(target[j] == target[i])
			{
				repickFlag = 1;
				break;
			}
		}

		if(repickFlag)
		{
			repickFlag = 0;
			i--;
		}
	}

	return target;
}

int numBaseball(int* targetNum)
{
	int playerGuess[4] = {0};

	printf("==================================================================================\n");
	printf("You should enter a four-digit number which has all different digits as your guess.\n");
	printf("==================================================================================\n\n");

	while(1)
	{
		char input[6] = {'\0'};
		int invalidInputFlag = 0;

		int strike = 0, ball = 0;
		int i, j;

		printf("Please enter your guess! : ");
		fgets(input, 6, stdin);

		/*
		int c;
		while((c = getchar()) != EOF)
		{
			printf("debug\n");
		} // clear the stdin
		*/

//		printf("debug2: input[4] is %c\n", input[4]);
		if(input[4] != '\n') invalidInputFlag = 1;

		for(i = 0; i < 4; i++)
		{
			if(input[i] < '0' || input[i] > '9')
			{
				invalidInputFlag = 1;
				break;
			}

			for(j = 0; j < i; j++)
			{
				if(input[j] == input[i])
					invalidInputFlag = 1;
			}

		}
		
		if(invalidInputFlag)
		{
			printf("Invalid input. Input must be a four-digit number and digits must be all different. \n");
			continue;
		}

		for(i = 0; i < 4; i++)
			playerGuess[i] = input[i] - '0';

		for(i = 0; i < 4; i++)
		{
			for(j = 0; j < 4; j++)
			{
				if(playerGuess[i] == targetNum[j])
				{
					if(i == j) strike++;
					else ball++;

					break;
				}
			}
		}

		if(strike == 4)
		{
			// 맞았어
			char restart;
			printf("\nCongratulations! You guessed the right answer! The answer is %1d%1d%1d%1d.\n\n", 
					targetNum[0], targetNum[1], targetNum[2], targetNum[3]);

			printf("Try again? (Y/N) : ");
			scanf("%c%*c", &restart);

			if(restart == 'Y' || restart == 'y') return 1;
			else return 0;
		}

		else
		{
			// 틀렸어
			printf("%d Strike(s), %d Ball(s)\n", strike, ball);
		}
	}
}
