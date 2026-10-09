import java.io.File;
import org.teavm.tooling.ConsoleTeaVMToolLog;
import org.teavm.tooling.TeaVMTargetType;
import org.teavm.tooling.TeaVMTool;
import org.teavm.vm.TeaVMOptimizationLevel;

// Usage: TeaVMBuild <main class> <output dir> <output file>
//
// Drives TeaVM's Wasm GC backend directly. TeaVM's CLI is not published for
// this version, and its Gradle plugin always puts the JSO plugin on the
// classpath, which would add JS imports to the module.
public final class TeaVMBuild {
    public static void main(String[] args) throws Exception {
        TeaVMTool tool = new TeaVMTool();
        tool.setTargetType(TeaVMTargetType.WEBASSEMBLY_GC);
        tool.setMainClass(args[0]);
        tool.setTargetDirectory(new File(args[1]));
        tool.setTargetFileName(args[2]);
        tool.setOptimizationLevel(TeaVMOptimizationLevel.FULL);
        tool.setObfuscated(true);
        // Strict mode adds explicit null and bounds checks that throw Java
        // exceptions, whose stack trace capture needs the JS runtime. Without
        // it, a bad access traps in the Wasm GC instruction instead.
        tool.setStrict(false);
        tool.setClassLoader(TeaVMBuild.class.getClassLoader());
        tool.setLog(new ConsoleTeaVMToolLog(false));
        tool.generate();
        var problems = tool.getProblemProvider().getSevereProblems();
        for (var p : problems) {
            System.err.println("error: " + p.getText() + " " + java.util.Arrays.toString(p.getParams()));
        }
        if (!problems.isEmpty()) {
            System.exit(1);
        }
    }
}
