import java.io.*;
import java.util.*;

public class NumberBaseball {
	private boolean playing;
	private ArrayList<Integer> targetNum;
	
	public NumberBaseball() {
		playing = true;
		targetNum = new ArrayList<Integer>();
			
		Random rand = new Random();
		int i = 0;
		
		while(i < 4) {
			int x = rand.nextInt(10);
			if(!targetNum.contains(x)) {
				targetNum.add(x);
				i++;
			}
		}
	}
	
	public static void main(String[] args) {
		boolean isPlaying = true;
		
		System.out.println("\n[Java] 숫자 야구");
		
		try {
			while(isPlaying) {
				NumberBaseball nb = new NumberBaseball();
				isPlaying = nb.play();
			}
		} catch (Exception e) {
			e.printStackTrace();
		}
	}
	
	public boolean play() throws IOException{
		
		BufferedReader br = new BufferedReader(new InputStreamReader(System.in));
		
		System.out.println("\n============================================================");
		System.out.println("서로 다른 숫자로 각 자리가 이루어진 네 자리 수를 입력하세요!");
		System.out.println("============================================================\n");
		
		while (true) {
			System.out.print("추측한 숫자를 입력해주세요. : ");
			String s = br.readLine();
			
			ArrayList<Integer> guess = new ArrayList<Integer>();
			
			try {	
				if(!s.matches("^[0-9]{4}$")) throw new IOException();
				
				for(int i = 0; i < 4; i++) {
					int x = Character.getNumericValue(s.charAt(i));	
					
					if(guess.contains(x)) throw new IOException();
					else guess.add(x);
				}
			} catch (IOException e) {
				System.out.println("입력이 잘못되었습니다. 입력은 서로 다른 숫자로 각 자리가 이루어진 네 자리 수여야 합니다.");
				continue;
			}
			
			int strike = 0, ball = 0;
			
			for(int i = 0; i < 4; i++) {
				if(targetNum.contains(guess.get(i))) {
					if(targetNum.get(i) == guess.get(i)) strike += 1;
					else ball += 1;
				}
			}
			
			if(strike == 4) {
				System.out.format("\n정답입니다! 답은 %d%d%d%d 입니다.\n",
						targetNum.get(0), targetNum.get(1), targetNum.get(2), targetNum.get(3));
				
				System.out.print("다시 플레이하시겠습니까? (Y/N) : ");
				String s1 = br.readLine();
				
				if(s1.equals("Y") || s1.equals("y")) playing = true;
				else playing = false;
				
				break;
			} else {
				System.out.format("%d 스트라이크, %d 볼입니다.\n", strike, ball);
			}
		}
		
		return playing;
	}
}
