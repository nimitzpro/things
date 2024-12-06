package main

import (
	"fmt"
	"strings"
	// "strconv"
)

func main() {
	fmt.Println("hello world")

	爆発 := "ヤっはろー"
	fmt.Println(爆発)

	クリア(&爆発)
	if len(爆発) > 0 {

		panic("explosion")
	}

	a := "miiro"
	b := "海色"
	// b := "みいろ"
	fmt.Println(eq(a, b))

}

func romaji(kana string) string {
	平仮名マップ := map[string]string{"あ": "a", "い": "i", "ろ": "ro", "み": "mi", "海": "mi", "色": "iro"}
	// kanji_map := map[string]string{"海": ["umi", "kai", "mi"]}
	var romanised string = ""
	// for _, char := range kana {
	// 	romanised += 平仮名マップ[string(char.([]byte))]
	// }

	slice := strings.Split(kana, "")
	len := len(slice)
	for i := 0; i < len; i++ {
		fmt.Printf("%s ", slice[i])
		romanised += 平仮名マップ[slice[i]]
	}
	fmt.Println("pain, " + romanised)
	return romanised
}

func eq(a string, b string) bool {
	return a == romaji(b)
}

func クリア(あ *string) {
	*あ = ""

}
