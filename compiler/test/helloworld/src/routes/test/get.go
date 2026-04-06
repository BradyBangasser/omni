// Package routes
package routes

type OmniContext struct{}

func GET(hi OmniContext) string {
	return "Hello World"
}
