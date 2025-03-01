# Ticket Assistant  
A bot that searches a repository’s issues for potential duplicates.  

---

## Current Functionality  
For now, it acts as a Fibonacci bot that extracts numbers from a pull request (PR) and calculates their Fibonacci values.  

---

## How to Use  

### Add it to Your Workflow  
```yml
  steps:
    - name: Compute Fibbot
      uses: @marcjazz/ticket-assistant@v1
      with: 
        enable_fib: true
        max_threshold: 100
```
**Fixes:**  
- "pontential" → "potential"  
- "threshhold" → "threshold"  
- Removed unnecessary `-` before parameters in YAML (they should be key-value pairs).  

### Run it Locally  
To run the action locally, clone the repository:  

```shell
git clone https://github.com/Marcjazz/ticket-assistant.git
```

#### Build and Run Using Docker  
Run the following command:  
- `true` enables Fibonacci computation.  
- `100` sets the maximum threshold.  

```shell
make start true 100
```
**Fixes:**  
- Clarified explanation of arguments.  
- Added missing parameter `100` to match expected usage.  

---

## Testing  
Run the following command to execute test cases:  
```shell
cargo test
```

---

## License  
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.  
