package main

import (
	"fmt"
	"testing"
)

func TestInputParseBasic(t *testing.T) {
	tests := []struct {
		input string
		want  requestObject
	}{
		{"GET https://meain.io", requestObject{url: "https://meain.io", method: GET}},
		{"POST https://meain.io", requestObject{url: "https://meain.io", method: POST}},
		{"POST meain.io", requestObject{url: "meain.io", method: POST}},
	}
	for i, tc := range tests {
		t.Run(fmt.Sprintf("InputParse=%d", i), func(t *testing.T) {
			got := parseInput(tc.input)
			if got != tc.want {
				t.Fatalf("got %v; want %v", got, tc.want)
			} else {
				t.Logf("Success !")
			}

		})
	}
}


func (suite *TestSuite) TestNewThing(){
	// blah blah blah
}