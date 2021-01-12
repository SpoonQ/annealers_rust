(function() {var implementors = {};
implementors["annealers"] = [{"text":"impl&lt;NodeType&gt; Send for FixedSingleQuadricModel&lt;NodeType&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;NodeType as SingleNode&gt;::RealType: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;S&gt; Send for FixedSingleProdIter&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;S&gt; Send for FixedSingleNeighborIter&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;Single, Key&gt; Send for DiscreteNode&lt;Single, Key&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Key: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;R&gt; Send for Spin&lt;R&gt;","synthetic":true,"types":[]},{"text":"impl&lt;R&gt; Send for Binary&lt;R&gt;","synthetic":true,"types":[]},{"text":"impl&lt;R&gt; Send for TwoVal&lt;R&gt;","synthetic":true,"types":[]},{"text":"impl Send for BinaryRepr","synthetic":true,"types":[]},{"text":"impl&lt;NodeType&gt; Send for SingleSolution&lt;NodeType&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;NodeType as SingleNode&gt;::RealType: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;G, P&gt; Send for AsStructuredSolverGeneratorWrapper&lt;G, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;G: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;P: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl !Send for UnstructuredEdgeIter","synthetic":true,"types":[]},{"text":"impl Send for Quadric","synthetic":true,"types":[]},{"text":"impl Send for HighOrder","synthetic":true,"types":[]}];
implementors["classical_solver"] = [{"text":"impl Send for NoneError","synthetic":true,"types":[]},{"text":"impl&lt;R&gt; Send for BetaType&lt;R&gt;","synthetic":true,"types":[]},{"text":"impl&lt;P&gt; Send for SimulatedAnnealerGenerator&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;P as SingleModel&gt;::NodeType: SingleNode,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;P as SingleModel&gt;::NodeType as SingleNode&gt;::RealType: Send,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;P, R&gt; Send for SimulatedAnnealer&lt;P, R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: Send,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;P as FixedSingleModel&gt;::NodeType: SingleNode,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;P as FixedSingleModel&gt;::NodeType as SingleNode&gt;::RealType: Send,&nbsp;</span>","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()