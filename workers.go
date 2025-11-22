package main

import (
	"sync/atomic"

	"github.com/veandco/go-sdl2/img"
	"github.com/veandco/go-sdl2/sdl"
)

type WorkerTask struct {
	filename   string
	index      int
	generation uint64
}

type WorkerResult struct {
	is_thumbnail bool
	surface      *sdl.Surface
	index        int
	err          error
}

const MAX_TASKS = 10

var thumbnailChan chan WorkerTask
var taskChan chan WorkerTask = make(chan WorkerTask, MAX_TASKS)
var resultChan chan WorkerResult = make(chan WorkerResult, MAX_TASKS)
var generation uint64

func LoadingWorker(tasks <-chan WorkerTask, results chan<- WorkerResult) {
	for task := range tasks {
		if task.generation == atomic.LoadUint64(&generation) {
			surface, err := img.Load(task.filename)
			results <- WorkerResult{
				is_thumbnail: false,
				index:        task.index,
				surface:      surface,
				err:          err,
			}
		}
	}
}

func ThumbnailWorker(tasks <-chan WorkerTask, results chan<- WorkerResult) {
	for task := range tasks {
		surface, err := img.Load(task.filename)
		results <- WorkerResult{
			is_thumbnail: true,
			index:        task.index,
			surface:      surface,
			err:          err,
		}
	}
}

func LoadImage() {
	taskChan <- WorkerTask{
		filename:   files[current_index].filename,
		index:      current_index,
		generation: atomic.AddUint64(&generation, 1),
	}
}

func LoadThumbnail(index int) {
	thumbnailChan <- WorkerTask{
		filename:   files[index].thumbnailFile,
		index:      index,
		generation: 0,
	}
}
