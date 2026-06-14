# Investigate Root Cause

When a bug surfaces, investigate the system design — never patch the symptom
by overwriting a file or applying a local fix. Trace the root cause through
the full mechanism before proposing any fix. Ask "why didn't the existing
mechanism handle this?" not "how do I manually fix the output?"

Never claim something "might be fixed" or "should work now" without verifying
the actual state first. When the user reports a bug, diagnose it fully and
propose a concrete fix in one message — never redirect the diagnosis back by
asking what the fix should be.

When the user asks for something to be codified as a rule or test, do it
immediately in the same session — do not defer.
