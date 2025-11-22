package main

import (
	"github.com/veandco/go-sdl2/img"
	"github.com/veandco/go-sdl2/sdl"
)

func ShowError(err error) {
	sdl.ShowSimpleMessageBox(sdl.MESSAGEBOX_ERROR, "Error", err.Error(), nil)
}

func main() {
	if err := sdl.Init(sdl.INIT_VIDEO); err != nil {
		ShowError(err)
		return
	}
	defer sdl.Quit()

	window, err := sdl.CreateWindow(
		"imvi",
		sdl.WINDOWPOS_CENTERED,
		sdl.WINDOWPOS_CENTERED,
		800,
		600,
		sdl.WINDOW_RESIZABLE)
	if err != nil {
		ShowError(err)
		return
	}
	defer window.Destroy()

	renderer, err := sdl.CreateRenderer(window, -1, sdl.RENDERER_ACCELERATED|sdl.RENDERER_PRESENTVSYNC)
	if err != nil {
		ShowError(err)
		return
	}
	defer renderer.Destroy()

	renderer.SetDrawColor(23, 36, 42, 255)

	if err := img.Init(img.INIT_JPG | img.INIT_PNG | img.INIT_TIF | img.INIT_WEBP); err != nil {
		ShowError(err)
		return
	}
	defer img.Quit()

	texture, err := img.LoadTexture(renderer, "test_data/test.jpg")
	if err != nil {
		ShowError(err)
		return
	}
	defer texture.Destroy()

	running := true
	for running {
		for event := sdl.PollEvent(); event != nil; event = sdl.PollEvent() {
			switch event.(type) {
			case *sdl.QuitEvent:
				running = false
			}
		}

		renderer.Clear()

		renderer.Copy(texture, nil, nil)

		renderer.Present()

		sdl.Delay(5)
	}
}
