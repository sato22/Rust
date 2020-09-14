/*
 C言語
 配列の数値の平均値を計算
*/

#include <stdio.h>

int main() {

  int i, n;
 
  float num[100];
  float sum, ave;
 
  printf("数値の数を入力(1〜100) = ");
  scanf("%d",&n);
 
  /* 数値の入力と総和の計算 */
  sum = 0.0;
  for( i=0;i<n;++i ) {
    printf("%d個目 = ",i+1);
    scanf("%f",&num[i]);
    sum += num[i];
  }
 
  /* 平均値の計算と出力 */
  ave = sum/n;
  printf("数値の平均値 = %.2f\n", ave);
 
  return 0;
}
