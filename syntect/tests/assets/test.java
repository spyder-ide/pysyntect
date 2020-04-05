
import java.util.Set;
import java.util.Map;
import java.util.List;
import java.util.Stack;
import java.util.Arrays;
import java.util.HashMap;
import java.util.HashSet;
import java.util.ArrayList;
import java.util.LinkedList;
import java.io.BufferedReader;
import java.io.InputStreamReader;
import java.util.stream.Collectors;

public class ProblemaC
{

    private static class Tuple<X, Y>
    {
          public final X x;
          public final Y y;

          public Tuple(X x, Y y)
          {
              this.x = x;
              this.y = y;
          }

          @Override
          public String toString() {
              return "(" + x + "," + y + ")";
          }

          @SuppressWarnings("unchecked")
	       @Override
          public boolean equals(Object other) {
              if (other == null) {
                  return false;
              }
              if (other == this) {
                  return true;
              }
              if (!(other instanceof Tuple)){
                  return false;
              }
              Tuple<X,Y> other_ = ((Tuple<X,Y>) other);
              return other_.x == this.x && other_.y == this.y;
          }

          @Override
          public int hashCode()
          {
              final int prime = 31;
              int result = 1;
              result = prime * result + ((x == null) ? 0 : x.hashCode());
              result = prime * result + ((y == null) ? 0 : y.hashCode());
              return result;
          }

    }

    public static Tuple<Integer, Integer> getUnvisited(int v,
                                                       HashMap<Integer, Set<Integer>> G,
                                                       HashMap<Tuple<Integer, Integer>, Boolean> marks)
    {
         Tuple<Integer, Integer> e = null;
         for(int w : G.get(v))
         {
              Tuple<Integer, Integer> t = new Tuple<Integer, Integer>(v, w);
              Tuple<Integer, Integer> t_inv = new Tuple<Integer, Integer>(w, v);
              if(!marks.get(t))
              {
                   marks.put(t, true);
                   marks.put(t_inv, true);
                   e = t;
                   break;
              }
         }
         return e;
    }

    public static LinkedList<Integer> hierholzer(int s,
                                                HashMap<Integer, Set<Integer>> G,
                                                HashMap<Tuple<Integer, Integer>, Boolean> marks)
    {
         Stack<Tuple<Integer, Integer>> forward = new Stack<Tuple<Integer, Integer>>();
         Stack<Tuple<Integer, Integer>> backward = new Stack<Tuple<Integer, Integer>>();
         LinkedList<Integer> path = new LinkedList<Integer>();
         Tuple<Integer, Integer> e = getUnvisited(s, G, marks);
         while(e != null)
         {
              forward.push(e);
              e = getUnvisited(e.y, G, marks);
         }
         while(!forward.isEmpty())
         {
              e = forward.pop();
              backward.push(e);
              e = getUnvisited(e.x, G, marks);
              while(e != null)
              {
                  forward.push(e);
                  e = getUnvisited(e.y, G, marks);
              }
         }
         while(!backward.isEmpty())
         {
             e = backward.pop();
             path.add(e.x);
         }
         path.add(e.y);
         return path;
    }

    public static void main(String... args)
    {
        try(BufferedReader br = new BufferedReader(new InputStreamReader(System.in)))
		  {
            String name;
            while((name = br.readLine()) != null)
            {
               int N = Integer.parseInt(br.readLine());
               HashMap<Integer, Set<Integer>> G = new HashMap<Integer, Set<Integer>>();
               HashMap<Tuple<Integer, Integer>, Boolean> marks = new HashMap<Tuple<Integer, Integer>, Boolean>();
               for(int i = 0; i < N; i++)
               {
                   String[] info = br.readLine().split(" ");
                   int n = Integer.parseInt(info[0]);
                   for(int j = 1; j < info.length; j++)
                   {
                       int num = Integer.parseInt(info[j]);
                       Set<Integer> l = G.get(n);
                       Set<Integer> l2 = G.get(num);
                       if(l == null)
                       {
                           l = new HashSet<Integer>();
                           G.put(n, l);
                       }
                       if(l2 == null)
                       {
                           l2 = new HashSet<Integer>();
                           G.put(num, l2);
                       }
                       l.add(num);
                       l2.add(n);
                       marks.put(new Tuple<Integer, Integer>(n, num), false);
                       marks.put(new Tuple<Integer, Integer>(num, n), false);
                   }
               }
               boolean can = true;
               int odd = 0;
               int odd_start = -1;
               for(Integer e : G.keySet())
               {
                   if(G.get(e).size() % 2 != 0)
                   {
                        odd_start = e;
                        odd++;
                   }
                   if(odd > 2)
                   {
                       can = false;
                       break;
                   }
               }
               if(can)
               {
                   int start = -1;
                   if(odd != 0)
                   {
                       start = odd_start;
                   }
                   else
                   {
                       start = G.keySet().stream().collect(Collectors.toList()).get(0);
                   }
                   LinkedList<Integer> path = hierholzer(start, G, marks);
                   String out = path.toString();
                   System.out.println(name +": "+(path.size()-1));
                   System.out.println(out.substring(1, out.length()-1).replace(", ", " "));
               }
               else
               {
                   System.out.println(name +": "+0);
                   System.out.println("*");
               }
           }
        }
        catch(Exception e)
        {
            e.printStackTrace();
        }
    }

}


