import { useForm } from "react-hook-form";
import { Task } from "../../interfaces/task";
import { TaskAPI } from "../../api";

const TaskForm = () => {
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<Task>();

  const onSubmit = async (data: Task) => {
    await TaskAPI.createTask(data);
    window.location.reload();
  };

  return (
    <>
      <form
        onSubmit={handleSubmit(onSubmit)}
      >
        <input
          className="border border-sky-500"
          id="text"
          {...register("text", { required: true })}
        >
        </input>
        {errors.text && (
          <span className="text-red-600">This field is required</span>
        )}
        <button type="submit">
          create
        </button>
      </form>
    </>
  );
};

export default TaskForm;
