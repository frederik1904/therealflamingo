watch:
	cd therealflamingo && cargo watch -x run
run_ui:
	cd therealflamingo-ui && HOST=0.0.0.0 npm run dev