/*
 C言語
 1から1000までの素数を表示
 */
 
#include <stdio.h>

int main(){
  int i, j;
 
  int flag;     // フラグ
 
  // 素数かどうか判定 
  for( i=2;i<=1000;++i ) {
    flag = 0;
    for( j=2;j<i;++j ){
      if( i%j==0 ) {
        flag = 1;
        break;
      }
    }
    // 判定結果を出力
    if( flag==0 )
      printf("%d ", i);
  }
  
  printf("\n");
  return 0;
}
