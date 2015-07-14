using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace NumberBaseball
{
    class Program
    {
        static void Main(string[] args)
        {
            bool isPlaying = true;
            Console.WriteLine("\n[C#] 숫자 야구");

            while(isPlaying)
            {
                NumBaseball nb = new NumBaseball();
                isPlaying = nb.Play();
            }
        }
    }

    class NumBaseball
    {
        private List<int> targetNum;
        private bool playing;

        public NumBaseball()
        {
            Random rand = new Random();

            playing = true;
            targetNum = new List<int>();

            int i = 0;
            while (i < 4)
            {
                int x = rand.Next(10);
                if (!targetNum.Contains(x))
                {
                    targetNum.Add(x);
                    i++;
                }
            }
        }

        public bool Play()
        {
            Console.WriteLine ("\n=============================================================");
            Console.WriteLine ("서로 다른 숫자로 각 자리가 이루어진 네 자리 수를 입력하세요.");
            Console.WriteLine ("=============================================================");

            while(true)
            {
                List<int> guess = new List<int>();

                Console.Write("\n추측한 숫자를 입력해주세요. : ");
                string s = Console.ReadLine();

                try
                {
                    if (s.Length != 4) throw new Exception();
                    char[] cs = s.ToCharArray();
                    for (int i = 0; i < 4; i++)
                    {
                        if (!Char.IsDigit(cs[i])) throw new Exception();

                        int x = (int)Char.GetNumericValue(cs[i]);
                        if (guess.Contains(x)) throw new Exception();
                        else guess.Add(x);
                    }
                } 
                catch (Exception e)
                {
                    Console.WriteLine("입력이 잘못되었습니다. 입력은 네 자리의 서로 다른 숫자로 이루어진 수여야 합니다.");
                    continue;
                }

                int strike = 0, ball = 0;

                for (int i = 0; i < 4; i++)
                {
                    int x = guess[i];
                    if (targetNum.Contains(x))
                    {
                        if (targetNum[i] == x) strike++;
                        else ball++;
                    }
                }

                if (strike == 4)
                {
                    Console.WriteLine("\n정답입니다! 답은 {0}{1}{2}{3} 입니다.\n",
                        targetNum[0], targetNum[1], targetNum[2], targetNum[3]);

                    Console.Write("다시 하시겠습니까? (Y/N) : ");
                    char c = Console.ReadKey().KeyChar;

                    if (c == 'Y' || c == 'y') playing = true;
                    else playing = false;
                    break;
                }
                else
                {
                    Console.WriteLine("{0} 스트라이크, {1} 볼이에요!", strike, ball);
                }
            }
            Console.WriteLine("");
            return playing;
        }
    }
}
