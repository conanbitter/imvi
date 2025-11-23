package main

import (
	"math"

	"github.com/veandco/go-sdl2/sdl"
)

const TILE_SIZE int = 200
const SCROLL_SIZE int = 100

var colCount = 1
var rowCount = 1
var rowCountVisible = 1
var maxYOffset = 1

var gridYOffset = 0
var gridXOffset = 0

var gridMode bool = false

func GetTileSize(width int, height int) (int, int, int, int) {
	var w, h int
	if width > height {
		w = TILE_SIZE
		h = height * TILE_SIZE / width
	} else if width < height {
		w = width * TILE_SIZE / height
		h = TILE_SIZE
	} else {
		w = TILE_SIZE
		h = TILE_SIZE
	}
	return w, h, (TILE_SIZE - w) / 2, (TILE_SIZE - h) / 2
}

func UpdateGridSize() {
	colCount = windowWidth / TILE_SIZE
	rowCount = int(math.Ceil(float64(len(files)) / float64(colCount)))
	rowCountVisible = int(math.Ceil(float64(windowHeight)/float64(TILE_SIZE))) + 1
	gridYOffset = 0
	gridXOffset = (windowWidth - TILE_SIZE*colCount) / 2
	maxYOffset = rowCount*TILE_SIZE - windowHeight
	//fmt.Println(colCount, rowCountVisible)
}

func DrawGrid(renderer *sdl.Renderer) {
	startRow := gridYOffset / TILE_SIZE
	itemOffset := startRow*TILE_SIZE - gridYOffset
	index := startRow * colCount

	var rect sdl.Rect

	for y := 0; y < rowCountVisible; y++ {
		if index >= len(files) {
			break
		}
		for x := 0; x < colCount; x++ {
			if index >= len(files) {
				break
			}
			rect.W = int32(files[index].tileWidth)
			rect.H = int32(files[index].tileHeight)
			rect.X = int32(files[index].tileX + TILE_SIZE*x + gridXOffset)
			rect.Y = int32(itemOffset + files[index].tileY + TILE_SIZE*y)
			renderer.Copy(files[index].thumbnail, nil, &rect)
			index++
		}
	}

}

func ScrollGrid(down bool) {
	if down {
		gridYOffset += SCROLL_SIZE
		if gridYOffset > maxYOffset {
			gridYOffset = maxYOffset
		}
	} else {
		gridYOffset -= SCROLL_SIZE
		if gridYOffset < 0 {
			gridYOffset = 0
		}
	}
}
