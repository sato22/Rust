/*
 * C言語のサンプルプログラム - Webkaru
 * - 入力した文字列の文字数を表示（出力） -
 */
#include <stdio.h>
 
int main()
{
 
  int i;
 
  /* 文字列 */
  char moji[100];
 
  /* 文字列の入力 */
  printf("文字列を入力してください = ");
  scanf("%s", moji);
 
  /* 文字数をカウント */
  for(i=0; moji[i]!='\0'; ++i);
 
  /* 文字数を出力 */
  printf("入力した文字列は「%d」文字です。\n",i);
 
  return 0;
}
