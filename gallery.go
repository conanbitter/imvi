package main

import (
	"math"

	"github.com/veandco/go-sdl2/sdl"
)

const TILE_SIZE int = 200
const TILE_BORDER int = 2
const TILE_INNER_SIZE int = TILE_SIZE - TILE_BORDER*2
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
		w = TILE_INNER_SIZE
		h = height * TILE_INNER_SIZE / width
	} else if width < height {
		w = width * TILE_INNER_SIZE / height
		h = TILE_INNER_SIZE
	} else {
		w = TILE_INNER_SIZE
		h = TILE_INNER_SIZE
	}
	return w, h, (TILE_SIZE-w)/2 + TILE_BORDER, (TILE_SIZE-h)/2 + TILE_BORDER
}

func UpdateGridSize() {
	currentRow := gridYOffset / TILE_SIZE
	leftTile := currentRow * colCount

	colCount = windowWidth / TILE_SIZE
	rowCount = int(math.Ceil(float64(len(files)) / float64(colCount)))
	rowCountVisible = int(math.Ceil(float64(windowHeight)/float64(TILE_SIZE))) + 1

	gridXOffset = (windowWidth - TILE_SIZE*colCount) / 2
	maxYOffset = rowCount*TILE_SIZE - windowHeight

	currentRow = leftTile / colCount
	gridYOffset = TILE_SIZE * currentRow
	ClampGridOffset()
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

func ClampGridOffset() {
	if gridYOffset > maxYOffset {
		gridYOffset = maxYOffset
	}
	if gridYOffset < 0 {
		gridYOffset = 0
	}
}

func ScrollGrid(down bool) {
	if down {
		gridYOffset += SCROLL_SIZE
	} else {
		gridYOffset -= SCROLL_SIZE
	}
	ClampGridOffset()
}

func ScrollFromCurrent() {
	currentRow := current_index / colCount
	gridYOffset = TILE_SIZE*currentRow - (windowHeight-TILE_SIZE)/2
	ClampGridOffset()
}

func GetIndexFromXY(x, y int) int {
	if x < gridXOffset || x > gridXOffset+TILE_SIZE*colCount {
		return -1
	}
	col := (x - gridXOffset) / TILE_SIZE
	row := (y + gridYOffset) / TILE_SIZE
	index := col + row*colCount
	if index < 0 || index >= len(files) {
		return -1
	}
	return index
}
