package main

import (
	"sync/atomic"

	"github.com/veandco/go-sdl2/img"
	"github.com/veandco/go-sdl2/sdl"
)

type WorkerTask struct {
	filename     string
	index        int
	is_thumbnail bool
	generation   uint64
}

type WorkerResult struct {
	is_thumbnail bool
	surface      *sdl.Surface
	index        int
	err          error
}

const MAX_TASKS = 10

var taskChan chan WorkerTask = make(chan WorkerTask, MAX_TASKS)
var resultChan chan WorkerResult = make(chan WorkerResult, MAX_TASKS)
var generation uint64

func LoadingWorker(tasks <-chan WorkerTask, results chan<- WorkerResult) {
	for task := range tasks {
		if task.generation == atomic.LoadUint64(&generation) {
			surface, err := img.Load(task.filename)
			results <- WorkerResult{
				is_thumbnail: task.is_thumbnail,
				index:        task.index,
				surface:      surface,
				err:          err,
			}
		}
	}
}

func LoadImage() {
	taskChan <- WorkerTask{
		filename:     files[current_index].filename,
		index:        current_index,
		is_thumbnail: false,
		generation:   atomic.AddUint64(&generation, 1),
	}
}
