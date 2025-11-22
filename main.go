package main

import (
	"fmt"
	"io/fs"
	"path/filepath"
	"strings"

	"github.com/veandco/go-sdl2/img"
	"github.com/veandco/go-sdl2/sdl"
)

type FileEntry struct {
	name           string
	filename       string
	thumbnail_file string
	thumbnail      *sdl.Texture
}

var extentions map[string]bool = map[string]bool{
	".cur":  true,
	".ico":  true,
	".bmp":  true,
	".pnm":  true,
	".xpm":  true,
	".xcf":  true,
	".pcx":  true,
	".gif":  true,
	".jpg":  true,
	".jpeg": true,
	".tif":  true,
	".tiff": true,
	".png":  true,
	".tga":  true,
	".lbm":  true,
	".xv":   true,
	".webp": true,
}

var current_index int = 0
var current_texture *sdl.Texture = nil
var files []FileEntry = make([]FileEntry, 0)

var window *sdl.Window
var renderer *sdl.Renderer

func ShowError(err error) {
	sdl.ShowSimpleMessageBox(sdl.MESSAGEBOX_ERROR, "Error", err.Error(), nil)
}

func CleanTextures() {
	for i := range files {
		if files[i].thumbnail != nil {
			files[i].thumbnail.Destroy()
		}
	}
	if current_texture != nil {
		current_texture.Destroy()
	}
}

func ChangeImage() error {
	//var err error
	if current_texture != nil {
		current_texture.Destroy()
		current_texture = nil
	}
	/*current_texture, err = img.LoadTexture(renderer, files[current_index].filename)
	if err != nil {
		return err
	}*/
	LoadImage()
	window.SetTitle(fmt.Sprintf("[%d/%d] %s - imvi", current_index+1, len(files), files[current_index].name))
	return nil
}

func IndexNext() error {
	if current_index < len(files)-1 {
		current_index++
		return ChangeImage()
	}
	return nil
}

func IndexPrev() error {
	if current_index > 0 {
		current_index--
		return ChangeImage()
	}
	return nil
}

func main() {
	root := "test_data"
	err := filepath.WalkDir(root, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if path == root {
			return nil
		}
		if d.IsDir() {
			return filepath.SkipDir
		} else {
			ext := strings.ToLower(filepath.Ext(path))
			fmt.Println(ext)
			if ok := extentions[ext]; !ok {
				return nil
			}

			name := filepath.Base(path)
			fmt.Println(path, name)

			files = append(files, FileEntry{
				name:           name,
				filename:       path,
				thumbnail_file: filepath.Join(filepath.Dir(path), "_preview", name),
				thumbnail:      nil,
			})
		}
		return nil
	})
	if err != nil {
		ShowError(err)
		return
	}

	if err := sdl.Init(sdl.INIT_VIDEO); err != nil {
		ShowError(err)
		return
	}
	defer sdl.Quit()
	sdl.SetHint(sdl.HINT_RENDER_SCALE_QUALITY, "best")

	window, err = sdl.CreateWindow(
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

	renderer, err = sdl.CreateRenderer(window, -1, sdl.RENDERER_ACCELERATED|sdl.RENDERER_PRESENTVSYNC)
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

	defer CleanTextures()
	for i := range files {
		fmt.Printf("%d / %d\n", i, len(files))
		files[i].thumbnail, err = img.LoadTexture(renderer, files[i].thumbnail_file)
		if err != nil {
			ShowError(err)
			return
		}
	}

	defer close(taskChan)

	err = ChangeImage()
	if err != nil {
		ShowError(err)
		return
	}

	go LoadingWorker(taskChan, resultChan)

	running := true
	for running {
		for event := sdl.PollEvent(); event != nil; event = sdl.PollEvent() {
			switch e := event.(type) {
			case *sdl.QuitEvent:
				running = false
			case *sdl.KeyboardEvent:
				if e.Type == sdl.KEYDOWN {
					switch e.Keysym.Scancode {
					case sdl.SCANCODE_ESCAPE:
						running = false
					case sdl.SCANCODE_RIGHT:
						err = IndexNext()
						if err != nil {
							ShowError(err)
							return
						}
					case sdl.SCANCODE_LEFT:
						err = IndexPrev()
						if err != nil {
							ShowError(err)
							return
						}
					}
				}
			}
		}

		select {
		case result := <-resultChan:
			if result.err != nil {
				ShowError(err)
				return
			}
			if result.index == current_index {
				current_texture, err = renderer.CreateTextureFromSurface(result.surface)
			}
			result.surface.Free()
			if err != nil {
				ShowError(err)
				return
			}
		default:
		}

		renderer.Clear()

		if current_texture != nil {
			renderer.Copy(current_texture, nil, nil)
		} else if files[current_index].thumbnail != nil {
			renderer.Copy(files[current_index].thumbnail, nil, nil)
		}

		renderer.Present()

		sdl.Delay(5)
	}
}
