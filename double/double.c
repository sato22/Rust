#include <stdio.h>

int var(int i);                                 //関数のプロトタイプ宣言

int main(){
    int s = 0, n = 0;                           //変数の宣言と初期化
    printf("数値を入力してください\n");

    scanf("%d", &s);                            //数値の入力
    n = var(s);                                 //関数var()を呼び出し、引数sを2倍する
    printf("入力した数値の2倍は%dです\n", n);

    return 0;
}
 
int var(int i){
    int ans = 0;
    ans = i * 2;                                //変数を2倍する計算式
    return ans;                                 //ansを返す
}
