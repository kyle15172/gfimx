from typing import List, Tuple
from urllib.parse import unquote
import re

def validate_patterns(raw_patterns: str) -> None:
    """
    Extracts and validates Regex patterns from the provided string

    Parameters:
    -----------

      - raw_patterns - a string containing patterns to be extracted and validated

    Throws:
    -------

    If any pattern extracted in the string cannot be compiled, an Exception is thrown
    """
    
    state = 0
    url_state = 0

    pattern_list: List[str] = [] # list of extracted patterns

    cur_buffer: str = "" # stores a pattern
    url_buffer: str = "" # Stores a url code to be decoded


    # State machine to extract patterns
    for char in raw_patterns:
        if char == '"':
            if state == 0:
                state = 1 # start to read string into buffer
            elif state == 1:
                state = 0 # reset state to consume a new pattern
                pattern_list.append(cur_buffer)
                cur_buffer = ""
            elif state == 2:
                raise Exception(f"Error in parsing patterns {raw_patterns}")

        elif char == '%':
            if state == 0:
                raise Exception(f"Error in parsing patterns {raw_patterns}")
            elif state == 1:
                state = 2 # read characters into the url_buffer
                url_buffer += char
                url_state = 2
            elif state == 2:
                raise Exception(f"Error in parsing patterns {raw_patterns}")

        elif state == 1:
            cur_buffer += char

        elif state == 2:
            if url_state == 1:
                url_state = 0
                url_buffer += char
                cur_buffer += unquote(url_buffer)
                url_buffer = ""
                state = 1
            elif url_state == 2:
                url_state = 1
                url_buffer += char

    for pattern in pattern_list:
        try:
            re.compile(pattern) # validate pattern
        except Exception as e:
            raise Exception(f"Could not compile '{pattern}': {e}")