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
	name            string
	filename        string
	thumbnailFile   string
	thumbnail       *sdl.Texture
	thumbnailWidth  int
	thumbnailHeight int
	tileWidth       int
	tileHeight      int
	tileX           int
	tileY           int
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

var displayRect sdl.Rect
var windowWidth int = 800
var windowHeight int = 600
var windowAR float32 = float32(windowWidth) / float32(windowHeight)
var textureWidth int = 1
var textureHeight int = 1

var zooming bool = false
var mouseX int = 0
var mouseY int = 0

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

func ChangeImage() {
	if current_texture != nil {
		current_texture.Destroy()
		current_texture = nil
	}
	LoadImage()
	window.SetTitle(fmt.Sprintf("[%d/%d] %s - imvi", current_index+1, len(files), files[current_index].name))
	zooming = false
	UpdateDisplayRect()
}

func IndexNext() {
	if current_index < len(files)-1 {
		current_index++
		ChangeImage()
	}
}

func IndexPrev() {
	if current_index > 0 {
		current_index--
		ChangeImage()
	}
}

func FloatClamp01(val float32) float32 {
	if val > 1.0 {
		return 1.0
	} else if val < 0.0 {
		return 0.0
	} else {
		return val
	}
}

func GetRectCoords(rect *sdl.Rect, x int, y int) (fx float32, fy float32) {
	fx = FloatClamp01(float32(x-int(rect.X)) / float32(rect.W))
	fy = FloatClamp01(float32(y-int(rect.Y)) / float32(rect.H))
	return
}

func UpdateDisplayRect() {
	width := 1
	height := 1
	if current_texture != nil {
		width = textureWidth
		height = textureHeight
	} else if files[current_index].thumbnail != nil {
		width = files[current_index].thumbnailWidth
		height = files[current_index].thumbnailHeight
	}

	imageAR := float32(width) / float32(height)
	if imageAR < windowAR {
		displayRect.H = int32(windowHeight)
		displayRect.Y = 0
		displayRect.W = int32(float32(windowHeight) * imageAR)
		displayRect.X = (int32(windowWidth) - displayRect.W) / 2
	} else {
		displayRect.W = int32(windowWidth)
		displayRect.X = 0
		displayRect.H = int32(float32(windowWidth) / imageAR)
		displayRect.Y = (int32(windowHeight) - displayRect.H) / 2
	}

	if zooming {
		fx, fy := GetRectCoords(&displayRect, mouseX, mouseY)
		displayRect.W = int32(textureWidth)
		displayRect.H = int32(textureHeight)
		displayRect.X = int32(-fx * float32(textureWidth-windowWidth))
		displayRect.Y = int32(-fy * float32(textureHeight-windowHeight))
	}
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
			if ok := extentions[ext]; !ok {
				return nil
			}

			name := filepath.Base(path)

			files = append(files, FileEntry{
				name:          name,
				filename:      path,
				thumbnailFile: filepath.Join(filepath.Dir(path), "_preview", name),
				thumbnail:     nil,
			})
		}
		return nil
	})
	if err != nil {
		ShowError(err)
		return
	}

	thumbnailChan = make(chan WorkerTask, len(files))
	for i := range files {
		LoadThumbnail(i)
	}
	close(thumbnailChan)

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
		int32(windowWidth),
		int32(windowHeight),
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
	defer close(taskChan)

	ChangeImage()
	UpdateGridSize()

	go LoadingWorker(taskChan, resultChan)
	go ThumbnailWorker(thumbnailChan, resultChan)

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
						IndexNext()
					case sdl.SCANCODE_LEFT:
						IndexPrev()
					}
				}
			case *sdl.WindowEvent:
				if e.Event == sdl.WINDOWEVENT_RESIZED {
					windowWidth = int(e.Data1)
					windowHeight = int(e.Data2)
					UpdateDisplayRect()
					UpdateGridSize()
				}
			case *sdl.MouseWheelEvent:
				if e.Y != 0 {
					isDown := e.Y < 0
					if e.Direction == sdl.MOUSEWHEEL_FLIPPED {
						isDown = !isDown
					}
					if gridMode {
						ScrollGrid(isDown)
					} else {
						if isDown {
							IndexNext()
						} else {
							IndexPrev()
						}
					}
				}
			case *sdl.MouseButtonEvent:
				if e.Type == sdl.MOUSEBUTTONDOWN {
					switch e.Button {
					case sdl.BUTTON_LEFT:
						if current_texture != nil {
							zooming = !zooming
							mouseX = int(e.X)
							mouseY = int(e.Y)
							UpdateDisplayRect()
						}
					case sdl.BUTTON_RIGHT:
						gridMode = !gridMode
					}
				}
			case *sdl.MouseMotionEvent:
				if zooming {
					mouseX = int(e.X)
					mouseY = int(e.Y)
					UpdateDisplayRect()
				}
			}
		}

		limit := 10
		for limit > 0 {
			select {
			case result := <-resultChan:
				if result.err != nil {
					ShowError(err)
					return
				}
				if !result.is_thumbnail && result.index != current_index {
					break
				}
				texture, err := renderer.CreateTextureFromSurface(result.surface)
				w := int(result.surface.W)
				h := int(result.surface.H)
				result.surface.Free()
				if err != nil {
					ShowError(err)
					return
				}

				if result.is_thumbnail {
					files[result.index].thumbnail = texture
					files[result.index].thumbnailWidth = w
					files[result.index].thumbnailHeight = h
					tw, th, tx, ty := GetTileSize(w, h)
					files[result.index].tileWidth = tw
					files[result.index].tileHeight = th
					files[result.index].tileX = tx
					files[result.index].tileY = ty
					if result.index == 0 {
						fmt.Println(files[result.index])
					}

					if result.index == current_index {
						UpdateDisplayRect()
					}
					limit--
				} else {
					current_texture = texture
					textureWidth = w
					textureHeight = h
					UpdateDisplayRect()
					limit = 0
				}
			default:
				limit = 0
			}
		}

		renderer.Clear()

		if gridMode {
			DrawGrid(renderer)
		} else {
			if current_texture != nil {
				renderer.Copy(current_texture, nil, &displayRect)
			} else if files[current_index].thumbnail != nil {
				renderer.Copy(files[current_index].thumbnail, nil, &displayRect)
			}
		}

		renderer.Present()

		sdl.Delay(5)
	}
}
