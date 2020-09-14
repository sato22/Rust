/*
 C言語
 入力した2つの数値を交換
 */
 
#include<stdio.h>
  
int main(void)
{
  
  // 変数の宣言
  int a, b, tmp;
  
  // 2つの数値を入力
  printf("a = ");
  scanf("%d", &a);
 
  printf("b = ");
  scanf("%d", &b);
 
  // 数値を入れ替える
  tmp = b;
  b = a;
  a = tmp;
 
  printf("===== After =====\n");
  printf("a = %d\n", a);
  printf("b = %d\n", b);
 
  return 0;
}
